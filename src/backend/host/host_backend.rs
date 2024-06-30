use crate::backend::host::host_kernels;
use crate::backend::{host::host_storage::HostStorage, traits::Backend};

pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T> where T: Copy;

    type AddKernel = host_kernels::HostAddKernel;
}
