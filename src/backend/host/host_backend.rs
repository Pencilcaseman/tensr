use crate::backend::host::host_applicator::HostApplicator2;
use crate::backend::host::host_kernels::HostBinaryOp;
use crate::backend::op_traits::Applicator2;
use crate::backend::{
    host::host_storage::HostStorage,
    traits::{Backend, ScalarAccessor},
};

pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T> where T: Copy;
}
