use crate::backend::{
    host::host_storage::HostStorage,
    traits::{self, Backend},
};

pub struct HostBackend {}

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T>;
}
