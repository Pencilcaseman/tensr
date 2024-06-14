use crate::backend::host::host_storage::SIMD_WIDTH;
use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub trait HostBinaryOp<T> {
    fn apply_scalar(lhs: &T, rhs: &T) -> T;
}

pub trait HostBinaryOpSimd<T>: HostBinaryOp<T>
where
    T: SimdElement,
{
    fn apply_simd(
        lhs: &Simd<T, SIMD_WIDTH>,
        rhs: &Simd<T, SIMD_WIDTH>,
    ) -> Simd<T, SIMD_WIDTH>;
}

macro_rules! host_kernel {
    ($kernel_name: ident, $operation: tt) => {
        paste::paste! {
            pub struct [< Host $kernel_name Kernel >];

            impl<T> HostBinaryOp<T> for [< Host $kernel_name Kernel >]
            where
                T: Copy + std::ops::$kernel_name<T, Output = T>,
            {
                #[inline(always)]
                fn apply_scalar(lhs: &T, rhs: &T) -> T {
                    *lhs $operation *rhs
                }
            }

            impl<T> HostBinaryOpSimd<T> for [< Host $kernel_name Kernel >]
            where
                T: SimdElement + std::ops::$kernel_name<T, Output = T>,
                Simd<T, SIMD_WIDTH>: std::ops::$kernel_name<
                    Simd<T, SIMD_WIDTH>,
                    Output = Simd<T, SIMD_WIDTH>,
                >,
            {
                #[inline(always)]
                fn apply_simd(
                    lhs: &Simd<T, SIMD_WIDTH>,
                    rhs: &Simd<T, SIMD_WIDTH>,
                ) -> Simd<T, SIMD_WIDTH>
                where
                    LaneCount<SIMD_WIDTH>: SupportedLaneCount,
                    T: SimdElement,
                {
                    *lhs $operation *rhs
                }
            }
        }
    };
}

host_kernel!(Add, +);
host_kernel!(Sub, -);
host_kernel!(Mul, *);
host_kernel!(Div, /);
