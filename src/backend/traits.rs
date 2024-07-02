//! Traits and types for defining and implementing backends for arrays. This
//! includes Backend structs and storage types.

use crate::backend::op_traits;
use crate::backend::op_traits::BinaryOp;
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

    type AddKernel: op_traits::BinaryOp;
}

pub trait ContainerLength {
    fn len(&self) -> usize;
}

pub trait ContainerScalarType {
    type Scalar: Copy;
}

pub trait ContainerStorageType: ContainerLength + ContainerScalarType {
    type Storage: Storage;
}

pub trait ContainerBackendType: ContainerStorageType {
    type Backend: Backend;
}

/// A trait marking an object as a storage medium. It may or may not own the
/// data that it contains.
pub trait Storage:
    ContainerLength
    + ContainerScalarType
    + std::ops::Index<usize, Output = Self::Scalar>
    + std::ops::IndexMut<usize>
{
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

/// Allows access to (an evaluated) scalar result at a given index.
pub trait ScalarAccessor: ContainerLength + ContainerScalarType {
    /// Return the `index`'th element of a data container or wrapper
    fn get_scalar(&self, index: usize) -> Self::Scalar;

    /// Write a value to the `index`'th element of a data container or wrapper
    fn write_scalar(&mut self, value: Self::Scalar, index: usize);
}

pub trait RawAccessor: ContainerStorageType {
    /// Return a reference to the underlying storage
    unsafe fn get_raw(&self) -> &Self::Storage;

    /// Return a mutable reference to the underlying storage
    unsafe fn get_raw_mut(&mut self) -> &mut Self::Storage;
}

pub trait LazyArrayObject: ContainerStorageType + ContainerBackendType {}
