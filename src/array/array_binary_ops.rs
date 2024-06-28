use crate::backend::host::host_applicator::HostApplicator2;
use crate::backend::host::host_kernels::HostAddKernel;
use crate::backend::traits::OwnedStorage;
use crate::{
    array::array_base::ArrayBase,
    array::function_2::Function2RefRef,
    backend::{host::host_backend::HostBackend, traits},
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
    // type Output = ArrayBase<Backend, StorageType, Dimensions>;
    type Output = Function2RefRef<
        'a,
        HostApplicator2,
        HostAddKernel,
        ArrayBase<Backend, StorageType, Dimensions>,
        ArrayBase<Backend, StorageType, Dimensions>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageType, Dimensions>,
    ) -> Self::Output {
        // If contiguous, use Backend contiguous add

        // If not contiguous, optimise the strides (order of traversal does not matter, so long
        // as all values are accessed)

        // todo!()

        Self::Output::new(self, rhs)
    }
}
