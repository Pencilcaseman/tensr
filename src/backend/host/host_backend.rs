use crate::backend::host::host_applicator::HostApplicator2;
use crate::backend::host::host_kernels::HostBinaryOp;
use crate::backend::op_traits::Applicator2;
use crate::backend::{
    host::host_kernels::HostAddKernel,
    host::host_storage::HostStorage,
    op_traits,
    traits::{Backend, ContainerScalar, ScalarAccessor},
};

pub struct HostBackend;

impl Backend for HostBackend {
    type OwnedStorage<T> = HostStorage<T> where T: Copy;

    type Applicator2<Op, Lhs, Rhs, Out> = HostApplicator2<Op, Lhs, Rhs, Out> where
    Lhs: ScalarAccessor
        + ContainerScalar,
    Rhs: ScalarAccessor<Scalar = Lhs::Scalar>
        + ContainerScalar<Scalar = Lhs::Scalar>,
    Out: ScalarAccessor<Scalar = Lhs::Scalar>
        + ContainerScalar<Scalar = Lhs::Scalar>,
    Op: HostBinaryOp<Lhs::Scalar>;

    type AddKernel = HostAddKernel;
}
