use std::marker::PhantomData;

use crate::backend::traits;
// use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

// pub trait BinaryOp<Backend: traits::Backend> {
//     // type StorageType: crate::backend::traits::OwnedStorage;
//     // type Scalar;
//     // type SIMD;

//     // fn apply_contiguous(
//     //     lhs: &Self::StorageType,
//     //     rhs: &Self::StorageType,
//     //     out: &mut Self::StorageType,
//     // );

//     fn apply_scalar<T>(lhs: T, rhs: T) -> T
//     where
//         T: std::ops::Add<T, Output = T>;

//     fn apply_simd<T, const WIDTH: usize>(
//         lhs: Simd<T, WIDTH>,
//         rhs: Simd<T, WIDTH>,
//     ) -> Simd<T, WIDTH>
//     where
//         LaneCount<WIDTH>: SupportedLaneCount,
//         T: std::ops::Add<T, Output = T> + SimdElement,
//         Simd<T, WIDTH>: std::ops::Add<Simd<T, WIDTH>, Output = Simd<T, WIDTH>>;
// }

pub trait BinaryOp<Backend: traits::Backend> {}

pub trait BinaryFunction<Backend: traits::Backend, Op: BinaryOp<Backend>> {
    fn apply_contiguous<T>(
        lhs: &Backend::OwnedStorage<T>,
        rhs: &Backend::OwnedStorage<T>,
        out: &mut Backend::OwnedStorage<T>,
    );
}

// pub struct BinaryFunction<Backend: traits::Backend, Op: BinaryOp<Backend>> {
//     phantom_backend: PhantomData<Backend>,
//     phantom_op: PhantomData<Op>,
// }
