use crate::array::array_traits::GetWriteableBuffer;
use crate::array::function_2::Function2;
use crate::array::function_2::TensrFn2;
use crate::backend::host::host_backend::HostBackend;
use crate::backend::host::host_kernels;
use crate::backend::traits::Backend;
use crate::backend::traits::ContainerScalarType;
use crate::backend::traits::ScalarAccessor;
use crate::backend::types::TensrType;
use crate::{
    array::array_base::ArrayBase, backend::op_traits, backend::traits,
    dimension::dim::Dimension,
};
use std::borrow::Borrow;

impl<'a, Op, Lhs, Rhs> ScalarAccessor
    for TensrFn2<'a, HostBackend, Op, Lhs, Rhs>
where
    Op: host_kernels::HostBinaryOp<Lhs::Scalar>,
    Lhs: ScalarAccessor,
    Rhs: ScalarAccessor<Scalar = Lhs::Scalar>,
{
    #[inline(always)]
    fn get_scalar(&self, index: usize) -> Self::Scalar {
        Op::apply_scalar(self.lhs.get_scalar(index), self.rhs.get_scalar(index))
    }

    #[cold]
    #[inline(never)]
    #[track_caller]
    fn write_scalar(&mut self, _value: Self::Scalar, _index: usize) {
        panic!("Cannot write to a TensrFn2");
    }
}

impl<'a, Op, Lhs, Rhs, Out> Function2<Out>
    for TensrFn2<'a, HostBackend, Op, Lhs, Rhs>
where
    Op: host_kernels::HostBinaryOp<Lhs::Scalar>,
    Lhs: ScalarAccessor,
    Rhs: ScalarAccessor<Scalar = Lhs::Scalar>,
    Out: ScalarAccessor<Scalar = Lhs::Scalar>,
{
    fn apply(&self, out: &mut Out) {
        for i in 0..self.lhs.len() {
            out.write_scalar(
                Op::apply_scalar(
                    self.lhs.get_scalar(i),
                    self.rhs.get_scalar(i),
                ),
                i,
            );
        }
    }
}
