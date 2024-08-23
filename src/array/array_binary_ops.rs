// ArrayBase + Function
use crate::{
    array::{
        array_base::ArrayBase, array_traits::GetWriteableBuffer,
        function_2::TensrFn2,
    },
    backend::{op_traits, traits},
    dimension::dim::Dimension,
};

macro_rules! implement_arary_binary_op {
    ($op_type: literal, [$($array_op_types: literal),*]) => {
        $(
            tensr_proc_macros::generate_binary_op!(($op_type, $array_op_types));
        )*
    }
}

macro_rules! implement_arary_binary_ops {
    ([$($op_type: literal),*], [$($array_op_types: literal),*]) => {
        $(
            tensr_proc_macros::generate_binary_op!(([$op_type, $array_op_types]));
        )*
    }
}

// Array Array
tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, ArrayBase), (Own, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, ArrayBase), (Ref, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, ArrayBase), (Own, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, ArrayBase), (Ref, ArrayBase)]
));

// Array TensrFn2
tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, ArrayBase), (Own, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, ArrayBase), (Ref, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, ArrayBase), (Own, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, ArrayBase), (Ref, TensrFn2)]
));

// TensrFn2 Array
tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, TensrFn2), (Own, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, TensrFn2), (Ref, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, TensrFn2), (Own, ArrayBase)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, TensrFn2), (Ref, ArrayBase)]
));

// TensrFn2 TensrFn2
tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, TensrFn2), (Own, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Own, TensrFn2), (Ref, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, TensrFn2), (Own, TensrFn2)]
));

tensr_proc_macros::generate_binary_op!((
    ["Add", "add"],
    [(Ref, TensrFn2), (Ref, TensrFn2)]
));

// #[cfg_attr(rustfmt, rustfmt_skip)]
// macro_rules! repeat_for_function_types {
// // //     ($OpName: ident, $op_name: ident: $_: tt) => {
// () => {
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Ref, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Ref, ArrayBase)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Ref, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Ref, ArrayBase)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Ref, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Ref, TensrFn2)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Ref, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Ref, TensrFn2)]));
// };
// }
//
// repeat_for_function_types!();

// crate::repeat_binary_ops!(repeat_for_function_types);

// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Ref, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Ref, ArrayBase)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Ref, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Own, ArrayBase)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Ref, ArrayBase)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, ArrayBase), (Ref, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, ArrayBase), (Ref, TensrFn2)]));
//
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Own, TensrFn2), (Ref, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Own, TensrFn2)]));
// tensr_proc_macros::generate_binary_op!((["Add", "add"], [(Ref, TensrFn2), (Ref, TensrFn2)]));

// impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
// {
// }

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

// macro_rules! binary_op_impl {
//     (
//     ($Op_Name: ident, $op_name: ident),
//
//     (
//         $lifetime: tt,
//         $($lifetime_generics: tt),*
//     ),
//
//     ($lhs_type: ty, ($(($lhs_generics: ty,
//         (
//             $($lhs_generic_bounds: ty),*
//         )
//     )),*)),
//
//     ($rhs_type: ty, ($(($rhs_generics: ty,
//         (
//             $($rhs_generic_bounds: ty),*
//         )
//     )),*))) => {
//         paste::paste! {
//             impl<$($lifetime_generics,)* Backend, $($lhs_generics),*, $($rhs_generics),*>
//                 std::ops::$Op_Name<$rhs_type<Backend, $($rhs_generics),*>>
//                 for $lhs_type<Backend, $($lhs_generics),*>
//             where
//                 Backend: traits::Backend,
//                 $($lhs_generics : $($lhs_generic_bounds +)*,)*
//                 $($rhs_generics : $($rhs_generic_bounds +)*,)*
//
//             {
//                 type Output = TensrFn2<
//                     $lifetime,
//                     Backend,
//                     Backend::[< $Op_Name Kernel >],
//                     $lhs_type<Backend, $($lhs_generics),*>,
//                     $rhs_type<Backend, $($rhs_generics),*>,
//                 >;
//
//                 fn $op_name(
//                     self,
//                     rhs: $rhs_type<Backend, $($rhs_generics),*>,
//                 ) -> Self::Output {
//                     Self::Output::new(self, rhs)
//                 }
//             }
//         }
//     };
// }
//
// // ArrayBase + ArrayBase
// binary_op_impl!(
//     (Add, add),
//     ('static,),
//     (ArrayBase,
//         (
//             ( StorageTypeLhs, (traits::Storage) ),
//             ( NDimsLhs, (Dimension) )
//         )
//     ),
//     (ArrayBase,
//         (
//             ( StorageTypeRhs, (traits::Storage) ),
//             ( NDimsRhs, (Dimension) )
//         )
//     )
// );
//
// // ArrayBase + &ArrayBase
// binary_op_impl!(
//     (Add, add),
//     ('a, 'a),
//     (ArrayBase,
//         (
//             ( StorageTypeLhs, (traits::Storage) ),
//             ( NDimsLhs, (Dimension) )
//         )
//     ),
//     (&'a ArrayBase,
//         (
//             ( StorageTypeRhs, (traits::Storage) ),
//             ( NDimsRhs, (Dimension) )
//         )
//     )
// );
//
// // &ArrayBase + ArrayBase
// binary_op_impl!(
//     (Add, add),
//     ('a, 'a),
//     (&'a ArrayBase,
//         (
//             ( StorageTypeLhs, (traits::Storage) ),
//             ( NDimsLhs, (Dimension) )
//         )
//     ),
//     (ArrayBase,
//         (
//             ( StorageTypeRhs, (traits::Storage) ),
//             ( NDimsRhs, (Dimension) )
//         )
//     )
// );
//
// // &ArrayBase + &ArrayBase
// binary_op_impl!(
//     (Add, add),
//     ('a, 'a),
//     (&'a ArrayBase,
//         (
//             ( StorageTypeLhs, (traits::Storage) ),
//             ( NDimsLhs, (Dimension) )
//         )
//     ),
//     (&'a ArrayBase,
//         (
//             ( StorageTypeRhs, (traits::Storage) ),
//             ( NDimsRhs, (Dimension) )
//         )
//     )
// );
//
// // TensrFn2 + ArrayBase
// binary_op_impl!(
//     (Add, add),
//     ('static,),
//     (TensrFn2,
//         (
//             ( Op, (op_traits::BinaryOp) ),
//             ( Lhs, (GetWriteableBuffer) ),
//             ( Rhs, (GetWriteableBuffer) )
//         )
//     ),
//     (ArrayBase,
//         (
//             ( StorageTypeRhs, (traits::Storage) ),
//             ( NDimsRhs, (Dimension) )
//         )
//     )
// );

// ArrayBase + &ArrayBase
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
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
//         &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// &ArrayBase + ArrayBase
// impl<'a, Backend, StorageTypeLhs, NDimsLhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>
// where
//     Backend: traits::Backend,
//     StorageTypeLhs: traits::Storage,
//     NDimsLhs: Dimension,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
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
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         &'a ArrayBase<Backend, StorageTypeLhs, NDimsLhs>,
//         &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }

// TensrFn2 + ArrayBase
// impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for TensrFn2<'a, Backend, Op, Lhs, Rhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     Lhs: GetWriteableBuffer,
//     Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         TensrFn2<'a, Backend, Op, Lhs, Rhs>,
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
// // TensrFn2 + &ArrayBase
// impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for TensrFn2<'a, Backend, Op, Lhs, Rhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     Lhs: GetWriteableBuffer,
//     Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         TensrFn2<'a, Backend, Op, Lhs, Rhs>,
//         &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
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
// // &TensrFn2 + ArrayBase
// impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     Lhs: GetWriteableBuffer,
//     Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>,
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
// // &TensrFn2 + &ArrayBase
// impl<'a, Backend, Op, Lhs, Rhs, StorageTypeRhs, NDimsRhs>
//     std::ops::Add<&'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>>
//     for &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>
// where
//     Backend: traits::Backend,
//     Op: op_traits::BinaryOp,
//     Lhs: GetWriteableBuffer,
//     Rhs: GetWriteableBuffer<Buffer = Lhs::Buffer>,
//     StorageTypeRhs: traits::Storage,
//     NDimsRhs: Dimension,
// {
//     type Output = TensrFn2<
//         'a,
//         Backend,
//         Backend::AddKernel,
//         &'a TensrFn2<'a, Backend, Op, Lhs, Rhs>,
//         &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     >;
//
//     fn add(
//         self,
//         rhs: &'a ArrayBase<Backend, StorageTypeRhs, NDimsRhs>,
//     ) -> Self::Output {
//         Self::Output::new(self, rhs)
//     }
// }
