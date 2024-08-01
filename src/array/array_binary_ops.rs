// ArrayBase + Function
use crate::array::array_traits::GetWriteableBuffer;
use crate::array::function_2::TensrFn2;
use crate::backend::traits::Backend;
use crate::{
    array::array_base::ArrayBase, backend::op_traits, backend::traits,
    dimension::dim::Dimension,
};

// ArrayBase + ArrayBase
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
//     type Output = TensrFn2<
//         'static,
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

macro_rules! binary_op_impl {
    // ($lifetime: tt, ($lhs_type: ty, $($lhs_generics: tt),*), ($rhs_type: ty, $($rhs_generics: tt),*)) => {
    ($lifetime: tt, ($lhs_type: ty, ($(($lhs_generics: ty,

    (
        $($lhs_generic_bounds: ty),*
    )

    )),*)), ($rhs_type: ty, ($(($rhs_generics: ty,

    (
        $($rhs_generic_bounds: ty),*
    )

    )),*))) => {
        paste::paste! {
            impl<Backend, $($lhs_generics),*, $($rhs_generics),*>
                std::ops::Add<$rhs_type<Backend, $($rhs_generics),*>>
                for $lhs_type<Backend, $($lhs_generics),*>
            where
                Backend: traits::Backend,

                // StorageTypeLhs: traits::Storage,
                // NDimsLhs: Dimension,
                // StorageTypeRhs: traits::Storage,
                // NDimsRhs: Dimension,

                $($lhs_generics : $($lhs_generic_bounds +)*,)*
                $($rhs_generics : $($rhs_generic_bounds +)*,)*

            {
                type Output = TensrFn2<
                    $lifetime,
                    Backend,
                    Backend::AddKernel,
                    // ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
                    // ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,

                    $lhs_type<Backend, $($lhs_generics),*>,
                    $rhs_type<Backend, $($rhs_generics),*>,
                >;

                fn add(
                    self,
                    rhs: $rhs_type<Backend, $($rhs_generics),*>,
                ) -> Self::Output {
                    Self::Output::new(self, rhs)
                }
            }
        }
    };
}

// binary_op_impl!('static, (ArrayBase, ( StorageTypeLhs, NDimsLhs )), (ArrayBase, ( StorageTypeRhs, NDimsRhs )));

binary_op_impl!('static,
    (ArrayBase,
        (
            ( StorageTypeLhs, (traits::Storage) ),
            ( NDimsLhs, (Dimension) )
        )
    ),
    (ArrayBase,
        (
            ( StorageTypeRhs, (traits::Storage) ),
            ( NDimsRhs, (Dimension) )
        )
    )
);

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
