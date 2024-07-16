use crate::{
    array::array_traits::HasWriteableBuffer,
    backend::{host::host_backend::HostBackend, traits},
    dimension::{axes::Axes, dim::Dimension},
};

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
    pub const fn new(axes: Axes<NDims>, storage: StorageType) -> Self
    where
        StorageType: traits::OwnedStorage,
    {
        Self { axes, storage, phantom_backend: std::marker::PhantomData }
    }

    pub fn new_empty(shape: NDims) -> Self
    where
        StorageType: traits::OwnedStorage,
    {
        let storage = unsafe { StorageType::new_from_shape_uninit(&shape) };
        Self::new(Axes::<NDims>::new_with_default_stride(shape), storage)
    }

    pub const fn shape(&self) -> &NDims {
        &self.axes.shape
    }

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
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        self.storage[index]
    }

    fn write_scalar(&mut self, value: Self::Scalar, index: usize) {
        self.storage[index] = value;
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

impl<Backend, StorageType, NDims> traits::ContainerScalarType
    for ArrayBase<Backend, StorageType, NDims>
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

impl<Backend, StorageType, NDims> traits::ContainerStorageAccessor
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    fn get_storage(&self) -> &Self::Storage {
        &self.storage
    }

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

impl<Backend, StorageType, NDims> traits::LazyArrayObject
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
}

unsafe impl<Backend, StorageType, NDims> HasWriteableBuffer
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::OwnedStorage,
    NDims: Dimension,
{
    type Buffer = StorageType::Raw;

    unsafe fn get_buffer(&self) -> (Self::Buffer, usize) {
        (self.storage.get_raw(), self.storage.len())
    }

    unsafe fn set_buffer_no_free(&mut self) {
        self.storage.set_no_free();
    }
}
