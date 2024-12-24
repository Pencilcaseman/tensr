use num_traits::{One, Zero};

use crate::{
    array::traits::GetWriteableBuffer,
    backend::{host::host_backend::HostBackend, traits},
    dimension::{axes::Axes, dim::Dimension},
};

/// The base type for all arrays. This type should not be used directly -- it is
/// used through various type aliases to make the API more ergonomic.
#[allow(clippy::module_name_repetitions)]
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
    /// use tensr::array::base::ArrayBase;
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

    pub fn zeros(shape: NDims) -> Self
    where
        StorageType: traits::OwnedStorage,
        StorageType::Scalar: Zero,
    {
        let mut storage = unsafe { StorageType::new_from_shape_uninit(&shape) };
        storage.fill(StorageType::Scalar::zero());
        Self::new(Axes::<NDims>::new_with_default_stride(shape), storage)
    }

    pub fn ones(shape: NDims) -> Self
    where
        StorageType: traits::OwnedStorage,
        StorageType::Scalar: One,
    {
        let mut storage = unsafe { StorageType::new_from_shape_uninit(&shape) };
        storage.fill(StorageType::Scalar::one());
        Self::new(Axes::<NDims>::new_with_default_stride(shape), storage)
    }

    pub fn new_with(shape: NDims, value: StorageType::Scalar) -> Self
    where
        StorageType: traits::OwnedStorage,
    {
        let mut storage = unsafe { StorageType::new_from_shape_uninit(&shape) };
        storage.fill(value);
        Self::new(Axes::<NDims>::new_with_default_stride(shape), storage)
    }

    pub fn fill(&mut self, value: StorageType::Scalar) {
        self.storage.fill(value);
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

impl<Backend, StorageType, NDims> traits::ContainerLength
    for &ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    fn len(&self) -> usize {
        self.storage.len()
    }
}

impl<Backend, StorageType, NDims> traits::ContainerLength
    for &mut ArrayBase<Backend, StorageType, NDims>
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

impl<Backend, StorageType, NDims> traits::ContainerScalarType
    for &ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Scalar = StorageType::Scalar;
}

impl<Backend, StorageType, NDims> traits::ContainerScalarType
    for &mut ArrayBase<Backend, StorageType, NDims>
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

impl<Backend, StorageType, NDims> traits::ContainerStorageType
    for &ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Storage = StorageType;
}

impl<Backend, StorageType, NDims> traits::ContainerStorageType
    for &mut ArrayBase<Backend, StorageType, NDims>
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
}

impl<Backend, StorageType, NDims> traits::MutableContainerStorageAccessor
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
    }
}

impl<Backend, StorageType, NDims> traits::ContainerStorageAccessor
    for &ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage(&self) -> &Self::Storage {
        &self.storage
    }
}

impl<Backend, StorageType, NDims> traits::ContainerStorageAccessor
    for &mut ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage(&self) -> &Self::Storage {
        &self.storage
    }
}

impl<Backend, StorageType, NDims> traits::MutableContainerStorageAccessor
    for &mut ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_storage_mut(&mut self) -> &mut Self::Storage {
        &mut self.storage
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

impl<Backend, StorageType, NDims> traits::ContainerBackendType
    for &ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Backend = Backend;
}

impl<Backend, StorageType, NDims> traits::ContainerBackendType
    for &mut ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Backend = Backend;
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

impl<Backend, StorageType, NDims> GetWriteableBuffer
    for &ArrayBase<Backend, StorageType, NDims>
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

impl<Backend, StorageType, NDims> GetWriteableBuffer
    for &mut ArrayBase<Backend, StorageType, NDims>
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
        // We don't own it, but we can mutate it, so this is fine
        self.storage.get_buffer_and_set_no_free(self.storage.len())
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

impl<StorageType, NDims> traits::ScalarAccessor
    for &ArrayBase<HostBackend, StorageType, NDims>
where
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self.storage[index]
    }
}

impl<StorageType, NDims> traits::ScalarAccessor
    for &mut ArrayBase<HostBackend, StorageType, NDims>
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
    for &mut ArrayBase<HostBackend, StorageType, NDims>
where
    StorageType: traits::Storage,
    NDims: Dimension,
{
    #[inline(always)]
    fn write_scalar(&mut self, value: Self::Scalar, index: usize) {
        self.storage[index] = value;
    }
}
