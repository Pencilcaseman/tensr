use crate::backend::host::host_backend::HostBackend;
use crate::backend::op_traits;

pub trait HostBinaryOp<T>: op_traits::BinaryOp {
    fn apply_scalar(lhs: T, rhs: T) -> T;
}

macro_rules! host_kernel {
    ($kernel_name: ident, $operation: tt) => {
        paste::paste! {
            pub struct [< Host $kernel_name Kernel >];

            impl op_traits::BinaryOp for [< Host $kernel_name Kernel >] {}

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

/*impl<T> op_traits::ScalarKernel<T> for op_traits::AddKernel<HostBackend>
where
    T: Copy + std::ops::Add<T, Output = T>,
{
    #[inline(always)]
    fn apply_scalar(lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}*/

/*impl AddKernel<HostBackend> {
    #[inline(always)]
    pub fn apply_scalar<T>(lhs: T, rhs: T) -> T
    where
        T: Copy + std::ops::Add<T, Output = T>,
    {
        lhs + rhs
    }
}*/
