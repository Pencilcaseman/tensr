// ArrayBase + Function
use crate::array::array_traits::GetWriteableBuffer;
use crate::array::function_2::TensrFn2;
use crate::backend::traits::Backend;
use crate::backend::types::TensrType;
use crate::{
    array::array_base::ArrayBase, backend::op_traits, backend::traits,
    dimension::dim::Dimension,
};
use std::borrow::Borrow;

// // ArrayBase + Rhs
// impl<Backend, StorageType, NDims, Rhs> std::ops::Add<Rhs>
//     for ArrayBase<Backend, StorageType, NDims>
// where
//     Backend: traits::Backend,
//     StorageType: traits::Storage,
//     NDims: Dimension,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2OwnOwn<
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageType, NDims>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
//
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
//
// // &ArrayBase + &Rhs
// impl<'a, Backend, StorageType, NDims, Rhs> std::ops::Add<&'a Rhs>
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
//
// // Function2RefRef + &Rhs
// impl<'a, Backend, Op, FnLhs, FnRhs, Rhs> std::ops::Add<&'a Rhs>
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

// &ArrayBase + Rhs
// impl<'a, Backend, StorageType, NDims, Rhs> std::ops::Add<Rhs>
//     for &'a ArrayBase<Backend, StorageType, NDims>
// where
//     Backend: traits::Backend,
//     StorageType: traits::Storage,
//     NDims: Dimension,
//     Rhs: traits::LazyArrayObject,
// {
//     type Output = Function2RefOwn<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageType, NDims>,
//         Rhs,
//     >;
//
//     fn add(self, rhs: Rhs) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// &ArrayBase + &ArrayBase
// impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
// where
//     Backend: traits::Backend,
//     StorageTypeLhs: traits::Storage,
//     NDimsLhs: Dimension,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = Function2RefRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
//         ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
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

// &Function2RefRef + &ArrayBase
// impl<'a, Backend, Op, FnLhs, FnRhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     FnLhs: traits::LazyArrayObject,
//     FnRhs: traits::LazyArrayObject,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = Function2RefRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         Function2RefRef<'a, Backend, Op, FnLhs, FnRhs>,
//         ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||

// // ================= ArrayBase + * ===================
// // ArrayBase + ArrayBase
// impl<Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
// where
//     Backend: traits::Backend,
//     StorageTypeLhs: traits::Storage,
//     NDimsLhs: Dimension,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = Function2OwnOwn<
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
//         ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
//
// // ArrayBase + &ArrayBase
// impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
// where
//     Backend: traits::Backend,
//     StorageTypeLhs: traits::Storage,
//     NDimsLhs: Dimension,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = Function2OwnRef<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
//         ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
//
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
//
// // &ArrayBase + &Rhs
// impl<'a, Backend, StorageType, NDims, Rhs> std::ops::Add<&'a Rhs>
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
//
// // Function2RefRef + &Rhs
// impl<'a, Backend, Op, FnLhs, FnRhs, Rhs> std::ops::Add<&'a Rhs>
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

// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
// ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||

// ArrayBase + ArrayBase
impl<Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
where
    Backend: traits::Backend,
    StorageTypeLhs: traits::Storage,
    NDimsLhs: Dimension,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'static,
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
        ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// ArrayBase + &ArrayBase
impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
where
    Backend: traits::Backend,
    StorageTypeLhs: traits::Storage,
    NDimsLhs: Dimension,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
        &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &ArrayBase + ArrayBase
impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
where
    Backend: traits::Backend,
    StorageTypeLhs: traits::Storage,
    NDimsLhs: Dimension,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
        ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &ArrayBase + &ArrayBase
impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
where
    Backend: traits::Backend,
    StorageTypeLhs: traits::Storage,
    NDimsLhs: Dimension,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
        &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// TensrFn2 + ArrayBase
impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        TensrFn2<'a, Backend, Op, Lhs, Rhs>,
        ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// TensrFn2 + &ArrayBase
impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        TensrFn2<'a, Backend, Op, Lhs, Rhs>,
        &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &TensrFn2 + ArrayBase
impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>,
        ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}

// &TensrFn2 + &ArrayBase
impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
    std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
    for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: GetWriteableBuffer,
    Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
    StorageTypeRhs: traits::Storage,
    NDimsRhs: Dimension,
{
    type Output = TensrFn2<
        'a,
        Backend,
        Backend::AddKernel,
        &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>,
        &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    >;

    fn add(
        self,
        rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
    ) -> Self::Output {
        Self::Output::new(self, rhs)
    }
}
