/*use crate::backend::{
    host::host_backend::HostBackend, host::host_kernels::HostBinaryOp,
    traits::ContainerScalar, traits::ScalarAccessor,
};*/
// use std::marker::PhantomData;

use crate::array::function_2;
use crate::backend::host::host_backend::HostBackend;
use crate::backend::host::host_kernels;
use crate::backend::traits;

impl<'a, Op, Lhs, Rhs, Out> function_2::Function2<Out>
    for function_2::Function2RefRef<'a, HostBackend, Op, Lhs, Rhs>
where
    Op: host_kernels::HostBinaryOp<Lhs::Scalar>,
    Lhs: traits::LazyArrayObject + traits::ScalarAccessor,
    Rhs: traits::LazyArrayObject + traits::ScalarAccessor<Scalar = Lhs::Scalar>,
    Out: traits::ScalarAccessor<Scalar = Lhs::Scalar>,
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

/*pub struct HostApplicator2<Op, Lhs, Rhs, Out> {
    phantom_op: PhantomData<(Op, Lhs, Rhs, Out)>,
    phantom_lhs: PhantomData<Lhs>,
    phantom_rhs: PhantomData<Rhs>,
    phantom_out: PhantomData<Out>,
}*/

// The SIMD implementation is a ***proper subset*** of the scalar version, so
// we can implement this with specialisation and a default implementation.
// impl<Op, Lhs, Rhs, Out, T> Applicator2<Op, Lhs, Rhs, Out, T>
//     for HostApplicator2<Op>
// where
//     Op: HostBinaryOpSimd<T>,
//     Lhs: SimdAccessor<Scalar = T, SIMD = Simd<T, SIMD_WIDTH>>,
//     Rhs: SimdAccessor<Scalar = T, SIMD = Simd<T, SIMD_WIDTH>>,
//     Out: SimdAccessor<Scalar = T, SIMD = Simd<T, SIMD_WIDTH>>,
// {
//     fn apply_contiguous(lhs: &Lhs, rhs: &Rhs, out: &mut Out) {
//         #[cold]
//         #[inline(never)]
//         #[track_caller]
//         fn lhs_rhs(left: usize, right: usize) -> ! {
//             panic!("left hand size length (is {left}) does not equal right hand side length (is {right})");
//         }

//         #[cold]
//         #[inline(never)]
//         #[track_caller]
//         fn in_out(in_: usize, out: usize) -> ! {
//             panic!("input length (is {in_}) does not equal output length (is {out})");
//         }

//         let lhs_len = lhs.len();
//         let rhs_len = rhs.len();
//         let out_len = out.len();

//         if lhs_len != rhs_len {
//             lhs_rhs(lhs_len, rhs_len);
//         }

//         if lhs_len != out_len {
//             in_out(lhs_len, out_len);
//         }

//         let mut i = 0;
//         while i < out_len {
//             out.write_simd(Op::apply_simd(lhs.get_simd(i), rhs.get_simd(i)), i);
//             i += SIMD_WIDTH;
//         }

//         while i < out_len {
//             out.write_scalar(
//                 Op::apply_scalar(lhs.get_scalar(i), rhs.get_scalar(i)),
//                 i,
//             );
//             i += 1;
//         }
//     }
// }
