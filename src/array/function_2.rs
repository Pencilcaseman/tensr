use crate::backend::op_traits;
use crate::backend::traits;
use std::marker::PhantomData;

pub trait Function2<Out> {
    fn apply(&self, out: &mut Out);
}

pub struct Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    pub(crate) lhs: &'a Lhs,
    pub(crate) rhs: &'a Rhs,
    pub(crate) backend: PhantomData<Backend>,
    pub(crate) op_phantom: PhantomData<Op>,
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerLength
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    fn len(&self) -> usize {
        self.lhs.len()
    }
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerScalar
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Scalar = Lhs::Scalar;
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerStorage
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Storage = Lhs::Storage;
}

impl<'a, Backend, Op, Lhs, Rhs> traits::LazyArrayObject
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
}

impl<'a, Backend, Op, Lhs, Rhs> Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    pub fn new(lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self { lhs, rhs, backend: PhantomData, op_phantom: PhantomData }
    }
}
