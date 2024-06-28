use crate::backend::op_traits::BinaryOp;

pub trait HostBinaryOp<T>: BinaryOp {
    fn apply_scalar(lhs: T, rhs: T) -> T;
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
                fn apply_scalar(lhs: T, rhs: T) -> T {
                    lhs $operation rhs
                }
            }
        }
    };
}

host_kernel!(Add, +);
host_kernel!(Sub, -);
host_kernel!(Mul, *);
host_kernel!(Div, /);
