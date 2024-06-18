use crate::{
    backend::traits,
    dimension::{dim::Dimension, stride::Stride},
};
use std::marker::PhantomData;

pub struct Array<
    Backend: traits::Backend,
    StorageType: traits::Storage,
    DimType: Dimension,
> {
    dims: DimType,
    stride: Stride<DimType>,
    storage: StorageType,
    backend: PhantomData<Backend>,
}
