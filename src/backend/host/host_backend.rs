use crate::backend::host::host_kernels;
use crate::backend::{host::host_storage::HostStorage, traits::Backend};

/// The host backend for Tensr, which allows you to perform calculations on the
/// CPU, storing data in RAM. This is probably the most well-supported backend
/// with the fewest requirements.
///
/// This is almost certainly the fastest backend for small arrays and simple
/// calculations, but you may want to use a GPU backend for larger arrays.
pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T> where T: Copy;
    type AddKernel = host_kernels::HostAddKernel;
}
