use crate::backend::{
    host::{host_kernels, host_storage::HostStorage},
    traits::Backend,
};

macro_rules! kernel_repeater {
    ($name: ident, $_1: tt, $_2: tt) => {
        paste::paste! {
            type [< $name Kernel >] = host_kernels::[< Host $name Kernel >];
        }
    };
}

macro_rules! repeat_kernel_repeater {
    ($([$name: ident, $_1: tt, $_2: tt]),*) => {
        $(
            kernel_repeater!($name, $_1, $_2);
        )*
    }
}

/// The host backend for Tensr, which allows you to perform calculations on the
/// CPU, storing data in RAM. This is probably the most well-supported backend
/// with the fewest requirements.
///
/// This is almost certainly the fastest backend for small arrays and simple
/// calculations, but you may want to use a GPU backend for larger arrays.
pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T>
        = HostStorage<T>
    where
        T: Copy;
    crate::repeat_binary_ops!(kernel_repeater);
    // repeat_kernel_repeater!(crate:array_binary_ops!());
    // repeat_kernel_repeater!([Add, add, +]);
}
