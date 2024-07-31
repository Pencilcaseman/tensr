use crate::{
    array::array_traits::GetWriteableBuffer,
    backend::{host::host_backend::HostBackend, traits},
    dimension::{axes::Axes, dim::Dimension},
};

/// The base type for all arrays. This type should not be used directly -- it is
/// used through various type aliases to make the API more ergonomic.
pub struct ArrayBase<
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
> {
    pub(crate) axes: Axes<NDims>,
    pub(crate) storage: StorageType,
    pub(crate) phantom_backend: std::marker::PhantomData<Backend>,
}

impl<Backend, StorageType, NDims> ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    /// Create a new [`ArrayBase`] object with the given axes and storage.
    pub const fn new(axes: Axes<NDims>, storage: StorageType) -> Self
    where
        StorageType: traits::OwnedStorage,
    {
        Self { axes, storage, phantom_backend: std::marker::PhantomData }
    }

    /// Create a new [`ArrayBase`] object with allocated, but uninitialized
    /// storage.
    ///
    /// # Safety
    /// The values in the Array are not initialized, so will almost certainly
    /// contain garbage data. Every element in the array should be written to
    /// before being read from.
    ///
    /// # Example
    ///
    /// ```rust
    /// // TODO: Refactor some of this into a single prelude
    /// use tensr::array::array_base::ArrayBase;
    /// use tensr::dimension::dim::Dim2;
    /// use tensr::backend::host::host_storage::HostStorage;
    /// use tensr::backend::host::host_backend::HostBackend;
    /// use crate::tensr::backend::traits::ContainerLength;
    ///
    /// let mut array = unsafe { ArrayBase::<HostBackend, HostStorage<i32>, Dim2>::new_empty(Dim2::new([3, 4])) };
    /// assert_eq!(array.len(), 12);
    /// ```
    pub unsafe fn new_empty(shape: NDims) -> Self
    where
        StorageType: traits::OwnedStorage,
    {
        let storage = StorageType::new_from_shape_uninit(&shape);
        Self::new(Axes::<NDims>::new_with_default_stride(shape), storage)
    }

    /// Get the dimensions of the array
    pub const fn shape(&self) -> &NDims {
        &self.axes.shape
    }

    /// Get the strides of the array
    pub const fn strides(&self) -> &NDims {
        &self.axes.stride
    }
}

impl<StorageType, NDims> traits::ScalarAccessor
    for ArrayBase<HostBackend, StorageType, NDims>
where
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self.storage[index]
    }
}

impl<StorageType, NDims> traits::ScalarWriter
    for ArrayBase<HostBackend, StorageType, NDims>
where
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn write_scalar(&mut self, value: Self::Scalar, index: usize) {
        self.storage[index] = value;
    }
}

// TODO: Implement all traits for &'a mut
impl<'a, StorageType, NDims> traits::ScalarAccessor
    for &'a ArrayBase<HostBackend, StorageType, NDims>
where
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self.storage[index]
    }
}

impl<Backend, StorageType, NDims> traits::ContainerLength
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<'a, Backend, StorageType, NDims> traits::ContainerLength
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<Backend, StorageType, NDims> traits::ContainerScalarType
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Scalar = StorageType::Scalar;
}

impl<'a, Backend, StorageType, NDims> traits::ContainerScalarType
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Scalar = StorageType::Scalar;
}

impl<Backend, StorageType, NDims> traits::ContainerStorageType
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Storage = StorageType;
}

impl<'a, Backend, StorageType, NDims> traits::ContainerStorageType
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Storage = StorageType;
}

impl<Backend, StorageType, NDims> traits::ContainerStorageAccessor
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage(&self) -> &Self::Storage {
        &self.storage
    }

    #[inline(always)]
    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }
}

impl<'a, Backend, StorageType, NDims> traits::ContainerStorageAccessor
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage(&self) -> &Self::Storage {
        &self.storage
    }

    #[cold]
    #[inline(never)]
    #[track_caller]
    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        panic!("Cannot write to a &ArrayBase");
    }
}

impl<Backend, StorageType, NDims> traits::ContainerBackendType
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Backend = Backend;
}

impl<Backend, StorageType, NDims> traits::LazyArrayObject
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
}

impl<Backend, StorageType, NDims> GetWriteableBuffer
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage + GetWriteableBuffer,
    NDims: Dimension,
{
    type Buffer = StorageType::Buffer;

    #[inline(always)]
    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        // If we own the storage, we can do whatever we want (within reason)
        self.storage.get_buffer_and_set_no_free(len)
    }
}

impl<'a, Backend, StorageType, NDims> GetWriteableBuffer
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage + GetWriteableBuffer,
    NDims: Dimension,
{
    type Buffer = StorageType::Buffer;

    #[inline(always)]
    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        _: usize,
    ) -> Option<Self::Buffer> {
        // It is not safe to get a buffer from a &ArrayBase, as it could be used
        // somewhere else
        None
    }
}
