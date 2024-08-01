use crate::backend::host::host_kernels;
use crate::backend::{host::host_storage::HostStorage, traits::Backend};

macro_rules! kernel_repeater {
    ($name: ident, $_1: tt, $_2: tt) => {
        paste::paste! {
            type [< $name Kernel >] = host_kernels::[< Host $name Kernel >];
        }
    };
}

/// The host backend for Tensr, which allows you to perform calculations on the
/// CPU, storing data in RAM. This is probably the most well-supported backend
/// with the fewest requirements.
///
/// This is almost certainly the fastest backend for small arrays and simple
/// calculations, but you may want to use a GPU backend for larger arrays.
pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T> where T: Copy;
    crate::repeat_binary_ops!(kernel_repeater);
}
