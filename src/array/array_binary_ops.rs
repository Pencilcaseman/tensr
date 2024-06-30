use crate::backend::traits::Backend;
use crate::{
    array::array_base::ArrayBase, array::function_2::Function2RefRef,
    backend::op_traits, backend::traits, dimension::dim::Dimension,
};

// &ArrayBase + &ArrayBase
impl<'a, Backend, StorageType, NDims>
    std::ops::Add<&'a ArrayBase<Backend, StorageType, NDims>>
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
{
    type Output = Function2RefRef<
        'a,
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageType, NDims>,
        ArrayBase<Backend, StorageType, NDims>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageType, NDims>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}
