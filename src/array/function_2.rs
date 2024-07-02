use crate::backend::op_traits;
use crate::backend::traits;
use std::marker::PhantomData;
use tensr_proc_macros::generate_function_type;

pub trait Function2<Out> {
    fn apply(&self, out: &mut Out);
}

/*pub struct Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    pub(crate) lhs: &'a Lhs,
    pub(crate) rhs: &'a Rhs,
    pub(crate) backend: PhantomData<Backend>,
    pub(crate) op_phantom: PhantomData<Op>,
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerLength
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    fn len(&self) -> usize {
        self.lhs.len()
    }
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerScalarType
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Scalar = Lhs::Scalar;
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerStorageType
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Storage = Lhs::Storage;
}

impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerBackendType
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    type Backend = Backend;
}

impl<'a, Backend, Op, Lhs, Rhs> traits::LazyArrayObject
    for Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
}

impl<'a, Backend, Op, Lhs, Rhs> Function2RefRef<'a, Backend, Op, Lhs, Rhs>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Lhs: traits::LazyArrayObject,
    Rhs: traits::LazyArrayObject,
{
    pub fn new(lhs: &'a Lhs, rhs: &'a Rhs) -> Self {
        Self { lhs, rhs, backend: PhantomData, op_phantom: PhantomData }
    }
}*/

// macro_rules! function_2_impl {
//     ($(([$own_left: ident, $left_type: ty], [$own_right: ident, $right_type: ty])),*) => {
//         paste::paste! {
//             $(
//                 pub struct [< Function2 $own_left $own_right>] <'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     pub(crate) lhs: $left_type,
//                     pub(crate) rhs: $right_type,
//                     pub(crate) backend: PhantomData<Backend>,
//                     pub(crate) op_phantom: PhantomData<Op>,
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerLength
//                     for  [< Function2 $own_left $own_right>] <'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     fn len(&self) -> usize {
//                         self.lhs.len()
//                     }
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerScalarType
//                     for [< Function2 $own_left $own_right>] <'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     type Scalar = Lhs::Scalar;
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerStorageType
//                     for [< Function2 $own_left $own_right>] <'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     type Storage = Lhs::Storage;
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> traits::ContainerBackendType
//                     for [< Function2 $own_left $own_right>]<'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     type Backend = Backend;
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> traits::LazyArrayObject
//                     for [< Function2 $own_left $own_right>]<'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                 }
//
//                 impl<'a, Backend, Op, Lhs, Rhs> [< Function2 $own_left $own_right>]<'a, Backend, Op, Lhs, Rhs>
//                 where
//                     Backend: traits::Backend,
//                     Op: op_traits::BinaryOp,
//                     Lhs: traits::LazyArrayObject,
//                     Rhs: traits::LazyArrayObject,
//                 {
//                     pub fn new(lhs: $left_type, rhs: $right_type) -> Self {
//                         Self { lhs, rhs, backend: PhantomData, op_phantom: PhantomData }
//                     }
//                 }
//             )*
//         }
//     };
// }

// function_2_impl!(
//     ([Ref, &'a Lhs], [Ref, &'a Rhs]),
//     ([Own, Lhs], [Ref, &'a Rhs]),
//     ([Ref, &'a Lhs], [Own, Rhs])
//     // ([Own, Lhs], [Own, Rhs])
// );

generate_function_type!([Ref, Ref]);
