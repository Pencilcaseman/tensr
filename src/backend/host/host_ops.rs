use crate::backend::{
    host::host_storage::{self, SIMD_WIDTH},
    op_traits,
};
use std::simd::{Simd, SimdElement};

pub struct HostAdd;

impl<T> op_traits::BinaryOp<host_storage::HostStorage<T>> for HostAdd
where
    T: SimdElement + std::ops::Add<T, Output = T>,
    Simd<T, SIMD_WIDTH>:
        std::ops::Add<Simd<T, SIMD_WIDTH>, Output = Simd<T, SIMD_WIDTH>>,
{
    type Scalar = T;
    type StorageType = host_storage::HostStorage<T>;
    type SIMD = Simd<T, SIMD_WIDTH>;

    fn apply_contiguous(
        lhs: &Self::StorageType,
        rhs: &Self::StorageType,
        out: &mut Self::StorageType,
    ) {
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
            panic!("input length (is {in_}) does not equal outpu length (is {out})");
        }

        if lhs.length != rhs.length {
            lhs_rhs(lhs.length, rhs.length);
        }

        if lhs.length != out.length {
            in_out(lhs.length, out.length);
        }

        // Round length down to n * SIMD_WIDTH
        let simd_size = (lhs.length / SIMD_WIDTH) * SIMD_WIDTH;

        let mut i = 0;
        while i < simd_size {
            let simd_lhs = Self::SIMD::from_slice(&lhs[i..i + SIMD_WIDTH]);
            let simd_rhs = Self::SIMD::from_slice(&rhs[i..i + SIMD_WIDTH]);
            let simd_out = simd_lhs + simd_rhs;

            simd_out.copy_to_slice(&mut out[i..i + SIMD_WIDTH]);
            i += SIMD_WIDTH;
        }

        while i < lhs.length {
            out[i] = lhs[i] + rhs[i];
            i += 1;
        }
    }
}
