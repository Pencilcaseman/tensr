use crate::backend::host::host_applicator::HostApplicator2;
use crate::backend::host::host_kernels::HostAddKernel;
use crate::backend::traits::OwnedStorage;
use crate::{
    array::array_base::ArrayBase, array::function_2::Function2RefRef,
    backend::op_traits::Applicator2, backend::traits,
    dimension::dim::Dimension,
};

impl<'a, Backend, StorageType, Dimensions>
    std::ops::Add<&'a ArrayBase<Backend, StorageType, Dimensions>>
    for &'a ArrayBase<Backend, StorageType, Dimensions>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    Dimensions: Dimension,
{
    type Output = Function2RefRef<
        'a,
        Backend::Applicator2<
            Backend::AddKernel,
            StorageType,
            StorageType,
            StorageType,
        >,
        Backend::AddKernel,
        ArrayBase<Backend, StorageType, Dimensions>,
        ArrayBase<Backend, StorageType, Dimensions>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageType, Dimensions>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}
