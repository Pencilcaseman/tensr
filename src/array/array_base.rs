use crate::{
    backend::traits,
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
    pub fn new(axes: Axes<NDims>, storage: StorageType) -> Self
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

    pub fn shape(&self) -> &NDims {
        &self.axes.shape
    }

    pub fn strides(&self) -> &NDims {
        &self.axes.stride
    }
}
