use crate::{
    backend::traits,
    dimension::{shape::Shape, stride::Stride},
};
use std::marker::PhantomData;

pub struct Array<Backend: traits::Backend, Scalar> {
    shape: Shape,
    stride: Stride,
    storage: Backend::OwnedStorage<Scalar>,
    backend: PhantomData<Backend>,
}
