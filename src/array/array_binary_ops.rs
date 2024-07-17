use crate::backend::traits::Backend;
use crate::backend::types::TensrType;
use crate::{
    array::array_base::ArrayBase,
    array::function_2::{
        Function2OwnOwn, Function2OwnRef, Function2RefOwn, Function2RefRef,
    },
    backend::op_traits,
    backend::traits,
    dimension::dim::Dimension,
};
use std::borrow::Borrow;

// ArrayBase + Rhs
impl<Backend, StorageType, NDims, Rhs> std::ops::Add<Rhs>
    for ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
    Rhs: traits::LazyArrayObject,
{
    type Output = Function2OwnOwn<
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageType, NDims>,
        Rhs,
    >;

    fn add(self, rhs: Rhs) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &ArrayBase + Rhs
impl<'a, Backend, StorageType, NDims, Rhs> std::ops::Add<Rhs>
    for &'a ArrayBase<Backend, StorageType, NDims>
where
    Backend: traits::Backend,
    StorageType: traits::Storage,
    NDims: Dimension,
    Rhs: traits::LazyArrayObject,
{
    type Output = Function2RefOwn<
        'a,
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageType, NDims>,
        Rhs,
    >;

    fn add(self, rhs: Rhs) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

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

// // &ArrayBase + &Rhs
// impl<'a, Backend, StorageType, NDims, Rhs: TensrType> std::ops::Add<&'a Rhs>
//     for &'a ArrayBase<Backend, StorageType, NDims>
// where
//     Backend: traits::Backend,
//     StorageType: traits::Storage,
//     NDims: Dimension,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2RefRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageType, NDims>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: &'a Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// // ArrayBase + &Rhs
// impl<'a, Backend, StorageType, NDims, Rhs: TensrType> std::ops::Add<&'a Rhs>
//     for ArrayBase<Backend, StorageType, NDims>
// where
//     Backend: traits::Backend,
//     StorageType: traits::Storage,
//     NDims: Dimension,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2OwnRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageType, NDims>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: &'a Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// &Function2RefRef + &Rhs
// impl<'a, Backend, Op, FnLhs, FnRhs, Rhs: TensrType> std::ops::Add<&'a Rhs>
//     for &'a Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     FnLhs: traits::LazyArrayObject,
//     FnRhs: traits::LazyArrayObject,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2RefRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: &'a Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
//
// // Function2RefRef + &Rhs
// impl<'a, Backend, Op, FnLhs, FnRhs, Rhs: TensrType> std::ops::Add<&'a Rhs>
//     for Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     FnLhs: traits::LazyArrayObject,
//     FnRhs: traits::LazyArrayObject,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2OwnRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: &'a Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
