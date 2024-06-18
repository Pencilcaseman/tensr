use crate::backend::{
    host::host_backend::HostBackend,
    traits::{Backend, ContainerLength, OwnedStorage, ScalarAccessor, Storage},
};
use rayon::prelude::*;
use std::ptr::NonNull;

/// The number of bytes to align heap-allocated memory to. The largest
/// alignment (64 bytes) is required by AVX-512, so we use this by default.
/// Please create a pull request or an issue if there is a reasont to change
/// this value.
pub const MEM_ALIGN: usize = 64;

/// A non-null pointer which can be used in parallel blocks
pub struct HostNonNull<T>(pub NonNull<T>);
unsafe impl<T> Send for HostNonNull<T> {}
unsafe impl<T> Sync for HostNonNull<T> {}

impl<T> Clone for HostNonNull<T> {
    fn clone(&self) -> Self {
        HostNonNull { 0: self.0.clone() }
    }
}

impl<T> Copy for HostNonNull<T> {}

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
}

impl<T> Storage for HostStorage<T> {}

impl<T> OwnedStorage for HostStorage<T> {}

impl<T> HostStorage<T> {
    /// Create a new [`HostStorage`] object with [`length`] elements, all initialized to
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

            Self { ptr: HostNonNull(NonNull::new(data).unwrap()), length }
        }
    }

    /// Create a new [`HostStorage`] object with [`length`] elements, not initializing the
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
    pub unsafe fn new_uninit(length: usize) -> Self {
        let data =
            std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(
                length * core::mem::size_of::<T>(),
                MEM_ALIGN,
            ))
            .cast::<T>();

        Self { ptr: HostNonNull(NonNull::new(data).unwrap()), length }
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

impl<T> Backend for HostStorage<T> {
    type OwnedStorage<V> = HostStorage<V>;
}

impl<T> HostBackend for HostStorage<T> {}

impl<T> ContainerLength for HostStorage<T> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<T> ScalarAccessor for HostStorage<T>
where
    T: Copy,
{
    type Scalar = T;

    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self[index]
    }

    fn write_scalar(&mut self, value: Self::Scalar, index: usize) {
        self[index] = value;
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
    pub fn slice_mut_par_iter<'a>(
        &'a mut self,
        slice_size: usize,
    ) -> impl IndexedParallelIterator<Item = &'a mut [T]> + '_ {
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

impl<T> Drop for HostStorage<T> {
    fn drop(&mut self) {
        // If the length is zero, there is nothing to free
        if self.length > 0 {
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
