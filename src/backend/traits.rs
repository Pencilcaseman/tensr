use crate::backend::host::host_backend::HostBackend;

/// The [`Backend`] trait is used to mark structs as a valid backend for
/// calculations. Operators should be implemented for a given backend, and
/// must operate on (valid) data provided by the [`Backend::OwnedStorage`]
/// type.
pub trait Backend {
    /// A type representing an object which can allocate, manage and store
    /// memory for a given [`Backend`]. Elements are of type `T`.
    type OwnedStorage<T>: OwnedStorage;
}

/// A trait marking an object as a storage medium. It may or may not own the
/// data that it contains.
pub trait Storage: std::ops::Index<usize> + std::ops::IndexMut<usize> {}

/// A trait marking an object as owning the data it contains. If this is the
/// case, the data must be stored contiguously and must be paired with a
/// backend.
pub trait OwnedStorage: Storage {}

pub trait ContainerLength {
    fn len(&self) -> usize;
}

/// Allows access to (an evaluated) scalar result at a given index.
pub trait ScalarAccessor: HostBackend + ContainerLength {
    type Scalar: Copy;

    /// Return the `index`'th element of a data container or wrapper
    fn get_scalar(&self, index: usize) -> Self::Scalar;

    /// Write a value to the `index`'th element of a data container or wrapper
    fn write_scalar(&mut self, value: Self::Scalar, index: usize);
}
