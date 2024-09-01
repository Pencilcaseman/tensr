use std::marker::PhantomData;

use crate::{
    array::traits::GetWriteableBuffer,
    backend::{
        op_traits, traits,
        traits::{ContainerLength, ContainerScalarType},
    },
};

pub trait Function2<Out> {
    fn apply(&self, out: &mut Out);
}

pub struct TensrFn2<'a, Backend, Op, Lhs, Rhs> {
    pub(crate) lhs: Lhs,
    pub(crate) rhs: Rhs,
    op: PhantomData<Op>,
    backend: PhantomData<Backend>,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, Backend, Op, Lhs, Rhs> ContainerScalarType
    for TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: ContainerScalarType,
    Rhs: ContainerScalarType<Scalar = Lhs::Scalar>,
{
    type Scalar = Lhs::Scalar;
}

impl<'a, Backend, Op, Lhs, Rhs> ContainerScalarType
    for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: ContainerScalarType,
    Rhs: ContainerScalarType<Scalar = Lhs::Scalar>,
{
    type Scalar = Lhs::Scalar;
}

impl<'a, Backend, Op, Lhs, Rhs> ContainerLength
    for TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: ContainerLength,
    Rhs: ContainerLength,
{
    fn len(&self) -> usize {
        self.lhs.len()
    }
}

impl<'a, Backend, Op, Lhs, Rhs> TensrFn2<'a, Backend, Op, Lhs, Rhs> {
    pub const fn new(lhs: Lhs, rhs: Rhs) -> Self {
        Self {
            lhs,
            rhs,
            op: PhantomData,
            backend: PhantomData,
            lifetime: PhantomData,
        }
    }
}

impl<'a, Backend, Op, Lhs, Rhs> GetWriteableBuffer
    for TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
{
    type Buffer = Lhs::Buffer;

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        self.lhs
            .get_buffer_and_set_no_free(len)
            .or_else(|| self.rhs.get_buffer_and_set_no_free(len))
    }
}

impl<'a, Backend, Op, Lhs, Rhs> GetWriteableBuffer
    for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
{
    type Buffer = Lhs::Buffer;

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        _len: usize,
    ) -> Option<Self::Buffer> {
        None
    }
}

impl<'a, Backend, Op, Lhs, Rhs> GetWriteableBuffer
    for &'a mut TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
{
    type Buffer = Lhs::Buffer;

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        self.lhs
            .get_buffer_and_set_no_free(len)
            .or_else(|| self.rhs.get_buffer_and_set_no_free(len))
    }
}
