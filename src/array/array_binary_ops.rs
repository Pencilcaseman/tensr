use crate::backend::traits::Backend;
use crate::{
    array::array_base::ArrayBase, array::function_2::Function2RefRef,
    backend::op_traits, backend::traits, dimension::dim::Dimension,
};

// &ArrayBase + &ArrayBase
/*impl<'a, Backend, StorageType, NDims>
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
}*/

// &ArrayBase + &Rhs
impl<'a, Backend, StorageType, NDims, Rhs> std::ops::Add<&'a Rhs>
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
    Rhs: traits::LazyArrayObject,
{
    type Output = Function2RefRef<
        'a,
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageType, NDims>,
        Rhs,
    >;

    fn add(self, rhs: &'a Rhs) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &Function2RefRef + &Rhs
impl<'a, Backend, Op, FnLhs, FnRhs, Rhs> std::ops::Add<&'a Rhs>
    for &'a Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    FnLhs: traits::LazyArrayObject,
    FnRhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Output = Function2RefRef<
        'a,
        Backend,
        Backend::AddKernel,
        Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>,
        Rhs,
    >;

    fn add(self, rhs: &'a Rhs) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}
