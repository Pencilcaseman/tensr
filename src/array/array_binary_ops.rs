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
