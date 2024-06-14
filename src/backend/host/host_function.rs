use crate::backend::{
    host::{
        host_backend::HostBackend,
        host_kernels::{HostBinaryOp, HostBinaryOpSimd},
        host_storage::SIMD_WIDTH,
    },
    traits,
};
use std::{
    marker::PhantomData,
    simd::{Simd, SimdElement},
};

pub struct HostFunction<Op> {
    phantom_op: PhantomData<Op>,
}

impl<Op> HostFunction<Op> {
    pub fn apply_contiguous<T>(
        lhs: &<HostBackend as traits::Backend>::OwnedStorage<T>,
        rhs: &<HostBackend as traits::Backend>::OwnedStorage<T>,
        out: &mut <HostBackend as traits::Backend>::OwnedStorage<T>,
    ) where
        Op: HostBinaryOp<T>,
        T: std::ops::Add<T, Output = T>,
    {
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

        let mut i = 0;
        while i < lhs.length {
            out[i] = Op::apply_scalar(&lhs[i], &rhs[i]);
            i += 1;
        }
    }

    pub fn apply_contiguous_simd<'a, T>(
        lhs: &'a <HostBackend as traits::Backend>::OwnedStorage<T>,
        rhs: &'a <HostBackend as traits::Backend>::OwnedStorage<T>,
        out: &'a mut <HostBackend as traits::Backend>::OwnedStorage<T>,
    ) where
        Op: HostBinaryOpSimd<T>,
        T: SimdElement + std::ops::Add<T, Output = T>,
        Simd<T, SIMD_WIDTH>:
            std::ops::Add<Simd<T, SIMD_WIDTH>, Output = Simd<T, SIMD_WIDTH>>,
    {
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
            let simd_lhs =
                Simd::<T, SIMD_WIDTH>::from_slice(&lhs[i..i + SIMD_WIDTH]);
            let simd_rhs =
                Simd::<T, SIMD_WIDTH>::from_slice(&rhs[i..i + SIMD_WIDTH]);
            let simd_out = Op::apply_simd(&simd_lhs, &simd_rhs);

            simd_out.copy_to_slice(&mut out[i..i + SIMD_WIDTH]);
            i += SIMD_WIDTH;
        }

        while i < lhs.length {
            out[i] = Op::apply_scalar(&lhs[i], &rhs[i]);
            i += 1;
        }
    }
}
