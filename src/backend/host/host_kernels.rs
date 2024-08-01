use crate::backend::op_traits;

pub trait HostBinaryOp<T>: op_traits::BinaryOp {
    fn apply_scalar(lhs: T, rhs: T) -> T;
}

/// Generate a host kernel for a trivial binary operation, such as addition,
/// subtraction or bitwise operators.
macro_rules! host_binary_kernel {
    ($operation_name: ident, $name: ident, $operation: tt) => {
        paste::paste! {
            pub struct [< Host $operation_name Kernel >];

            impl op_traits::BinaryOp for [< Host $operation_name Kernel >] {}

            impl<T> HostBinaryOp<T> for [< Host $operation_name Kernel >]
            where
                T: Copy + std::ops::$operation_name<T, Output = T>,
            {
                #[inline(always)]
                fn apply_scalar(lhs: T, rhs: T) -> T {
                    lhs $operation rhs
                }
            }
        }
    };
}

crate::repeat_binary_ops!(host_binary_kernel);
