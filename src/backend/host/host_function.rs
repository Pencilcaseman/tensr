use crate::{
    array::function_2::{Function2, TensrFn2},
    backend::{
        host::{host_backend::HostBackend, host_kernels},
        traits::{ScalarAccessor, ScalarWriter},
    },
};

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
}

impl<'a, Op, Lhs, Rhs, Out> Function2<Out>
    for TensrFn2<'a, HostBackend, Op, Lhs, Rhs>
where
    Op: host_kernels::HostBinaryOp<Lhs::Scalar>,
    Lhs: ScalarAccessor,
    Rhs: ScalarAccessor<Scalar = Lhs::Scalar>,
    Out: ScalarAccessor<Scalar = Lhs::Scalar> + ScalarWriter,
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
