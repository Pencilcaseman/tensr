//! Traits and types for defining and implementing backends for arrays. This
//! includes Backend structs and storage types.

use crate::{backend::op_traits, dimension::dim::Dimension};

macro_rules! kernel_type_repeater {
    ($name: ident, $_1: tt, $_2: tt) => {
        paste::paste! {
            type [< $name Kernel >]: op_traits::BinaryOp;
        }
    };
}

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

    crate::repeat_binary_ops!(kernel_type_repeater);
}

/// This trait marks an object as being a container with a length, and
/// provides a method for accessing the length.
pub trait ContainerLength {
    /// Returns the length of the container
    fn len(&self) -> usize;

    /// Returns true if the container is empty. This is equivalent to
    /// `self.len() == 0`.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// This trait provides access to the scalar type of the container. For
/// example, a [`Vec<u32>`] has a scalar type of `u32`.
///
/// ## Note
/// Note that this trait may not be implemented for standard library
/// containers
pub trait ContainerScalarType {
    /// The scalar type of the container
    type Scalar: Copy;
}

/// This trait marks an object as having a storage type. For example,
/// an [`ArrayBase`] may have a storage type of [`HostStorage`], which
/// stores data on the host.
pub trait ContainerStorageType: ContainerLength + ContainerScalarType {
    /// The storage type of the container
    type Storage: Storage;
}

/// This trait allows access to the storage type of the container.
pub trait ContainerStorageAccessor: ContainerStorageType {
    /// Return a reference to the underlying storage
    fn get_storage(&self) -> &Self::Storage;
}

/// This trait allows mutable access to the storage type of the container.
pub trait MutableContainerStorageAccessor:
    ContainerStorageType + ContainerStorageAccessor
{
    /// Return a mutable reference to the underlying storage
    fn get_storage_mut(&mut self) -> &mut Self::Storage;
}

/// Marks a struct as depending on a given backend. Generally, only structs
/// with the same backend can be used together.
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
    /// The equivalent storage type, but which owns the data it stores
    type OwnedStorageType: OwnedStorage;

    /// Mark the data to not be freed when the main object is dropped. This is
    /// necessary for preventing invalid memory accesses when reusing the same
    /// storage object.
    ///
    /// # Safety
    /// The data MUST be freed elsewhere, otherwise a memory leak will occur.
    unsafe fn set_no_free(&mut self);
}

/// A trait marking an object as owning the data it contains. If this is the
/// case, the data must be stored contiguously and must be paired with a
/// backend.
pub trait OwnedStorage: Storage {
    /// The raw type of the data stored by this object. For example, this may
    /// be a pointer to the underlying data.
    type Raw;

    fn new_from_shape<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension,
        Self::Scalar: Default;

    unsafe fn new_from_shape_uninit<Dim>(shape: &Dim) -> Self
    where
        Dim: Dimension;

    /// Return a pointer to the underlying data.
    ///
    /// # Safety
    /// The caller must ensure that the pointer is valid for the lifetime of
    /// the object and that immutable data is not written to.
    unsafe fn get_raw(&self) -> Self::Raw;
}

/// Allows direct access to scalar values in a container.
pub trait ScalarAccessor: ContainerLength + ContainerScalarType {
    /// Return the `index`'th element of a data container or wrapper
    fn get_scalar(&self, index: usize) -> Self::Scalar;
}

/// Allows writing scalar values to a container.
pub trait ScalarWriter: ContainerLength + ContainerScalarType {
    /// Write a value to the `index`'th element of a data container or wrapper
    fn write_scalar(&mut self, value: Self::Scalar, index: usize);
}
