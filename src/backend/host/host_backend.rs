use crate::backend::{
    host::host_storage::HostStorage,
    traits::{self, Backend},
};

struct HostBackend {}

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T>;
}

impl traits::HostBackend for HostBackend {}
