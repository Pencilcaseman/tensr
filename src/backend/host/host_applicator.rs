use crate::backend::{
    host::host_kernels::HostBinaryOp, op_traits::Applicator2,
    traits::ContainerScalar, traits::ScalarAccessor,
};
use std::marker::PhantomData;

/*pub struct HostApplicator2<Op> {
    phantom_op: PhantomData<Op>,
}*/

pub struct HostApplicator2<Op, Lhs, Rhs, Out>
where
    Lhs: ContainerScalar,
    Rhs: ContainerScalar<Scalar = Lhs::Scalar>,
    Out: ContainerScalar<Scalar = Lhs::Scalar>,
    Op: HostBinaryOp<Lhs::Scalar>,
{
    phantom_op: PhantomData<Op>,
    phantom_lhs: PhantomData<Lhs>,
    phantom_rhs: PhantomData<Rhs>,
    phantom_out: PhantomData<Out>,
}

impl<Op, Lhs, Rhs, Out> Applicator2<Op, Lhs, Rhs, Out>
    for HostApplicator2<Op, Lhs, Rhs, Out>
where
    Op: HostBinaryOp<Lhs::Scalar>,
    Lhs: ScalarAccessor + ContainerScalar,
    Rhs: ScalarAccessor<Scalar = Lhs::Scalar>
        + ContainerScalar<Scalar = Lhs::Scalar>,
    Out: ScalarAccessor<Scalar = Lhs::Scalar>
        + ContainerScalar<Scalar = Lhs::Scalar>,
{
    fn apply_contiguous(lhs: &Lhs, rhs: &Rhs, out: &mut Out) {
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn lhs_rhs(left: usize, right: usize) -> ! {
            panic!("left hand size length (is {left}) does not equal right hand side length (is {right})");
        }

        #[cold]
        #[inline(never)]
        #[track_caller]
        fn in_out(in_: usize, out: usize) -> ! {
            panic!("input length (is {in_}) does not equal output length (is {out})");
        }

        let lhs_len = lhs.len();
        let rhs_len = rhs.len();
        let out_len = out.len();

        if lhs_len != rhs_len {
            lhs_rhs(lhs_len, rhs_len);
        }

        if lhs_len != out_len {
            in_out(lhs_len, out_len);
        }

        let mut i = 0;

        while i < out_len {
            out.write_scalar(
                Op::apply_scalar(lhs.get_scalar(i), rhs.get_scalar(i)),
                i,
            );
            i += 1;
        }
    }
}

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
