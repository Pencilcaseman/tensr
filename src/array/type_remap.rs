use super::base::ArrayBase;
use crate::{
    backend::host::{host_backend::HostBackend, host_storage::HostStorage},
    dimension::dim,
};

pub type Array1<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim1>;
pub type Array2<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim2>;
pub type Array3<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim3>;
pub type Array4<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim4>;
pub type Array5<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim5>;
pub type Array6<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim6>;
pub type Array7<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim7>;
pub type Array8<T> = ArrayBase<HostBackend, HostStorage<T>, dim::Dim8>;
