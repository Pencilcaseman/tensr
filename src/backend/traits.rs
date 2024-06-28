use crate::backend::op_traits::Applicator2;
use crate::dimension::dim::Dimension;

/// The [`Backend`] trait is used to mark structs as a valid backend for
/// calculations. Operators should be implemented for a given backend, and
/// must operate on (valid) data provided by the [`Backend::OwnedStorage`]
/// type.
pub trait Backend {
    /// A type representing an object which can allocate, manage and store
    /// memory for a given [`Backend`]. Elements are of type `T`.
    type OwnedStorage<T>: OwnedStorage
    where
        T: Copy;

    // type Applicator2<Op, Lhs, Rhs, Out, T>: Applicator2<Op, Lhs, Rhs, Out, T>;

    /*fn contiguous_apply2<Operation, Left, Right, Out, Op, Scalar>(
        lhs: &Left,
        rhs: &Right,
        out: &mut Out,
    ) where
        Operation: Applicator2<Op, Left, Right, Out, Scalar>,
        Left: ScalarAccessor<Scalar = Scalar>,
        Right: ScalarAccessor<Scalar = Scalar>,
        Out: ScalarAccessor<Scalar = Scalar>;*/
}

/// A trait marking an object as a storage medium. It may or may not own the
/// data that it contains.
pub trait Storage: std::ops::Index<usize> + std::ops::IndexMut<usize> {
    type Scalar: Copy;
}

/// A trait marking an object as owning the data it contains. If this is the
/// case, the data must be stored contiguously and must be paired with a
/// backend.
pub trait OwnedStorage: Storage {
    fn new_from_shape<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension,
        Self::Scalar: Default;

    unsafe fn new_from_shape_uninit<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension;
}

pub trait ContainerLength {
    fn len(&self) -> usize;
}

/// Allows access to (an evaluated) scalar result at a given index.
pub trait ScalarAccessor: ContainerLength {
    type Scalar: Copy;

    /// Return the `index`'th element of a data container or wrapper
    fn get_scalar(&self, index: usize) -> Self::Scalar;

    /// Write a value to the `index`'th element of a data container or wrapper
    fn write_scalar(&mut self, value: Self::Scalar, index: usize);
}
