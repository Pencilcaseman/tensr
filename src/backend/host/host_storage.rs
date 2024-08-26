use std::ptr::NonNull;

use rayon::prelude::*;

use crate::{
    array::array_traits::GetWriteableBuffer,
    backend::traits::{
        ContainerLength, ContainerScalarType, ContainerStorageType,
        OwnedStorage, ScalarAccessor, ScalarWriter, Storage,
    },
    dimension::dim::Dimension,
};

/// The number of bytes to align heap-allocated memory to. The largest
/// alignment (64 bytes) is required by AVX-512, so we use this by default.
/// Please create a pull request or an issue if there is a reason to change
/// this value.
pub const MEM_ALIGN: usize = 64;

/// A non-null pointer which can be used in parallel blocks
#[derive(Debug)]
pub struct HostNonNull<T>(pub NonNull<T>);
unsafe impl<T> Send for HostNonNull<T> {}
unsafe impl<T> Sync for HostNonNull<T> {}

impl<T> Copy for HostNonNull<T> {}

impl<T> Clone for HostNonNull<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// An [`OwnedStorage`] object for data in host memory
///
/// # Example
/// ```rust
/// use tensr::backend::host::host_storage::HostStorage;
///
/// let mut host_storage = HostStorage::<usize>::new(10);
/// assert_eq!(host_storage.length, 10);
///
/// for i in 0..host_storage.length {
///     host_storage[i] = i + 1;
/// }
///
/// assert_eq!(host_storage[0], 1);
/// assert_eq!(host_storage[9], 10);
///
/// assert_eq!(host_storage[2..6], [3, 4, 5, 6]);
/// assert_eq!(host_storage[6..=9], [7, 8, 9, 10]);
/// ```
pub struct HostStorage<T> {
    pub ptr: HostNonNull<T>,
    pub length: usize,
    pub free_on_drop: bool,
}

impl<T> Storage for HostStorage<T>
where
    T: Copy,
{
    type OwnedStorageType = Self;

    unsafe fn set_no_free(&mut self) {}
}

impl<T> ContainerLength for HostStorage<T> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<T> ContainerScalarType for HostStorage<T>
where
    T: Copy,
{
    type Scalar = T;
}

impl<T> ContainerStorageType for HostStorage<T>
where
    T: Copy,
{
    type Storage = Self;
}

impl<T> OwnedStorage for HostStorage<T>
where
    T: Copy,
{
    type Raw = HostNonNull<T>;

    fn new_from_shape<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension,
        Self::Scalar: Default,
    {
        Self::new(shape.len())
    }

    unsafe fn new_from_shape_uninit<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension,
    {
        Self::new_uninit(shape.len())
    }

    unsafe fn get_raw(&self) -> Self::Raw {
        self.ptr
    }
}

impl<T> HostStorage<T> {
    /// Create a new [`HostStorage`] object with `length` elements, all initialized to
    /// `T::default()`.
    ///
    /// # Example
    /// ```rust
    /// use tensr::backend::host::host_storage::HostStorage;
    ///
    /// let host_storage = HostStorage::<f32>::new(10);
    /// assert_eq!(host_storage.length, 10);
    ///
    /// for i in 0..host_storage.length {
    ///     assert_eq!(host_storage[i], 0.0);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the memory allocation fails
    #[must_use]
    pub fn new(length: usize) -> Self
    where
        T: Default,
    {
        unsafe {
            let data = std::alloc::alloc(
                std::alloc::Layout::from_size_align_unchecked(
                    length * core::mem::size_of::<T>(),
                    MEM_ALIGN,
                ),
            )
            .cast::<T>();

            // Initialise all elements to their default value
            for i in 0..length {
                *data.add(i) = T::default();
            }

            Self {
                ptr: HostNonNull(NonNull::new(data).unwrap()),
                length,
                free_on_drop: true,
            }
        }
    }

    /// Create a new [`HostStorage`] object with `length` elements, not initializing the
    /// memory. For trivial types, this might be fine, but for types which require
    /// construction, this may cause problems if you are not careful.
    ///
    /// # Example
    /// ```rust
    /// use tensr::backend::host::host_storage::HostStorage;
    ///
    /// let host_storage = unsafe { HostStorage::<f32>::new_uninit(10) };
    /// assert_eq!(host_storage.length, 10);
    /// ```
    ///
    /// # Safety
    ///
    /// Each element is uninitialized, and must be written to before being read
    ///
    /// # Panics
    ///
    /// Panics if the memory allocation fail
    #[must_use]
    pub unsafe fn new_uninit(length: usize) -> Self {
        let data =
            std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(
                length * core::mem::size_of::<T>(),
                MEM_ALIGN,
            ))
            .cast::<T>();

        Self {
            ptr: HostNonNull(NonNull::new(data).unwrap()),
            length,
            free_on_drop: false,
        }
    }

    pub fn take_as_vec(&mut self) -> Vec<T> {
        unsafe {
            // Set length to zero so we do not free data
            let length = self.length;
            self.length = 0;
            Vec::from_raw_parts(self.ptr.0.as_ptr(), length, length)
        }
    }
}

impl<T> HostStorage<T>
where
    T: Send + Sync,
{
    /// Create a parallel slice iterator over slices of length `slice_size`.
    ///
    /// If the length of the input is nto a multiple of the slice size, the
    /// remaining elements are ignored.
    #[must_use]
    pub fn slice_par_iter(
        &self,
        slice_size: usize,
    ) -> impl IndexedParallelIterator<Item = &[T]> + '_ {
        let simd_size = self.length / slice_size;
        (0..simd_size)
            .into_par_iter()
            .map(move |i| &self[i * slice_size..(i + 1) * slice_size])
    }

    /// Create a parallel mutable slice iterator with slices of length `slice_size`.
    ///
    /// If the length of the input is not a multiple of the slice size, the remaining
    /// elements are ignored.
    #[must_use]
    pub fn slice_mut_par_iter(
        &mut self,
        slice_size: usize,
    ) -> impl IndexedParallelIterator<Item = &mut [T]> + '_ {
        let elements = self.length / slice_size;
        (0..elements).into_par_iter().map_init(
            || self.ptr,
            move |ptr, i| {
                let start = i * slice_size;
                let end = (i + 1) * slice_size;

                unsafe {
                    std::slice::from_raw_parts_mut(
                        ptr.0.as_ptr().add(start),
                        end - start,
                    )
                }
            },
        )
    }
}

impl<T> ScalarAccessor for HostStorage<T>
where
    T: Copy,
{
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self[index]
    }
}

impl<T> ScalarWriter for HostStorage<T>
where
    T: Copy,
{
    fn write_scalar(&mut self, value: Self::Scalar, index: usize) {
        self[index] = value;
    }
}

impl<T> Drop for HostStorage<T> {
    fn drop(&mut self) {
        // If the length is zero, there is nothing to free
        if self.free_on_drop && self.length > 0 {
            // We can convert the data into a vec and drop that instead, so
            // the logic is handled by the STL
            drop(self.take_as_vec());
        }
    }
}

impl<T> std::ops::Index<usize> for HostStorage<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        #[cfg(debug_assertions)]
        if index >= self.length {
            assert_failed(index, self.length)
        }

        unsafe { self.ptr.0.as_ptr().add(index).as_ref().unwrap() }
    }
}

impl<T> std::ops::IndexMut<usize> for HostStorage<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        #[cfg(debug_assertions)]
        if index >= self.length {
            assert_failed(index, self.length)
        }

        unsafe { self.ptr.0.as_ptr().add(index).as_mut().unwrap() }
    }
}

impl<T> std::ops::Index<std::ops::Range<usize>> for HostStorage<T> {
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        if index.start >= self.length {
            assert_failed(index.start, self.length)
        }

        if index.end > self.length {
            assert_failed(index.end, self.length)
        }

        unsafe {
            std::slice::from_raw_parts(
                self.ptr.0.as_ptr().add(index.start),
                index.end - index.start,
            )
        }
    }
}

impl<T> std::ops::IndexMut<std::ops::Range<usize>> for HostStorage<T> {
    fn index_mut(
        &mut self,
        index: std::ops::Range<usize>,
    ) -> &mut Self::Output {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        if index.start >= self.length {
            assert_failed(index.start, self.length)
        }

        if index.end > self.length {
            assert_failed(index.end, self.length)
        }

        unsafe {
            std::slice::from_raw_parts_mut(
                self.ptr.0.as_ptr().add(index.start),
                index.end - index.start,
            )
        }
    }
}

impl<T> std::ops::Index<std::ops::RangeInclusive<usize>> for HostStorage<T> {
    type Output = [T];

    fn index(&self, index: std::ops::RangeInclusive<usize>) -> &Self::Output {
        let start = *index.start();
        let end = *index.end();

        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("index (is {index}) must be <= len (is {len})");
        }

        if start >= self.length {
            assert_failed(start, self.length)
        }

        if end >= self.length {
            assert_failed(*index.end(), self.length)
        }

        unsafe {
            std::slice::from_raw_parts(
                self.ptr.0.as_ptr().add(*index.start()),
                end - start + 1,
            )
        }
    }
}

impl<T> GetWriteableBuffer for HostStorage<T> {
    type Buffer = HostNonNull<T>;

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        if self.length >= len {
            self.free_on_drop = false;
            Some(self.ptr)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::hint::black_box;

    use super::*;

    macro_rules! test_all {
        ($macro_name:ident, $($type:ty),+) => {
            $(
                paste::paste! {
                    $macro_name!($type, [<$macro_name _ $type>]);
                }
            )+
        };
    }

    macro_rules! test_all_fundamental {
        ($macro_name:ident) => {
            test_all!($macro_name, i16, i32, i64, u16, u32, u64, f32, f64);
        };
    }

    macro_rules! test_alloc {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let n = 1000;
                let s = HostStorage::<$type>::new(n);
                assert_eq!(s.length, n);

                // Assert alignment is correct
                assert_eq!((s.ptr.0.as_ptr() as usize) % MEM_ALIGN, 0);

                for i in 0..s.length {
                    type Type = $type;
                    assert_eq!(s[i], { Type::default() });
                }
            }
        };
    }

    macro_rules! test_alloc_uninit {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let n = 1000;
                let mut s = unsafe { HostStorage::<$type>::new_uninit(n) };
                assert_eq!(s.length, n);

                // Assert alignment is correct
                assert_eq!((s.ptr.0.as_ptr() as usize) % MEM_ALIGN, 0);

                // Check we can write to this data without segfaulting
                for i in 0..s.length {
                    type Type = $type;
                    s[i] = Type::default();
                    assert_eq!(s[i], Type::default());
                }
            }
        };
    }

    macro_rules! test_take_as_vec {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let n = 1000;
                let mut v = Vec::new();

                // Drop s to check the memory is not freed
                {
                    let mut s = HostStorage::<$type>::new(n);
                    v = s.take_as_vec();
                    drop(s);
                }

                assert_eq!(v.len(), n);

                // Check all values are valid and correct
                for i in 0..v.len() {
                    type Type = $type;
                    assert_eq!(v[i], { Type::default() });
                }
            }
        };
    }

    macro_rules! test_slice_par_iter {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let slice_width = 4;
                let n_slices = 1000;
                let n = n_slices * slice_width;
                let s = HostStorage::<$type>::new(n);

                type Type = $type;

                (0..n)
                    .into_par_iter()
                    .zip(s.slice_par_iter(slice_width))
                    .for_each(|(_, slice)| {
                        for i in 0..slice_width {
                            assert_eq!(slice[i], Type::default());
                        }
                    });
            }
        };
    }

    macro_rules! test_slice_mut_par_iter {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let slice_width = 4;
                let n_slices = 1000;
                let n = n_slices * slice_width;
                let mut s = unsafe { HostStorage::<$type>::new_uninit(n) };

                type Type = $type;

                (0..n)
                    .into_par_iter()
                    .zip(s.slice_mut_par_iter(slice_width))
                    .for_each(|(_, slice)| {
                        for i in 0..slice_width {
                            slice[i] = Type::default();
                            assert_eq!(slice[i], Type::default());
                        }
                    });
            }
        };
    }

    macro_rules! test_drop {
        ($type:ty, $name:ident) => {
            #[test]
            fn $name() {
                let n = 8196;

                // Create a LOT of these and see if the system runs out of memory...
                for _ in 0..10_000 {
                    let s = black_box(HostStorage::<$type>::new(n));
                    drop(s);
                }
            }
        };
    }

    test_all_fundamental!(test_alloc);
    test_all_fundamental!(test_alloc_uninit);
    test_all_fundamental!(test_take_as_vec);
    test_all_fundamental!(test_slice_par_iter);
    test_all_fundamental!(test_slice_mut_par_iter);
    test_all_fundamental!(test_drop);
}
