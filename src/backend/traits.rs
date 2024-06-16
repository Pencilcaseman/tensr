use crate::backend::host::host_backend::HostBackend;
use std::simd::SimdElement;

/// The [`Backend`] trait is used to mark structs as a valid backend for
/// calculations. Operators should be implemented for a given backend, and
/// must operate on (valid) data provided by the [`Backend::OwnedStorage`]
/// type.
pub trait Backend {
    /// A type representing an object which can allocate, manage and store
    /// memory for a given [`Backend`]. Elements are of type `T`.
    type OwnedStorage<T>: OwnedStorage;
}

/// A trait marking an object as owning the data it contains. If this is the
/// case, the data must be stored contiguously and must be paired with a
/// backend.
pub trait OwnedStorage:
    std::ops::Index<usize> + std::ops::IndexMut<usize>
{
}

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

/// Allows access to (an evaluated) SIMD result at a given index.
pub trait SimdAccessor: ScalarAccessor<Scalar: SimdElement> {
    type SIMD;

    /// Return a SIMD packet starting at the `index`'th element of a data
    /// container or wrapper.
    ///
    /// Note the following:
    ///  - The elements returned will be 'contiguous' in the context of the
    ///    parent object (i.e. the values are sequential from the viewpoint of
    ///    the owner, not the storage medium)
    ///  - The index is treated as a scalar offset, so `get_simd(0)` and
    ///    `get_simd(1)` will overlap (assuming packet width > 1)
    fn get_simd(&self, index: usize) -> Self::SIMD;

    /// Write a SIMD packet to the `index`'th `n` elements of a data
    /// container or wrapper.
    ///
    /// Note the following:
    ///  - The elements written will be 'contiguous' in the context of the
    ///    parent object (i.e. the values are sequential from the viewpoint of
    ///    the owner, not the storage medium)
    ///  - The index is treated as a scalar offset, so `x.write_simd(..., 0)` and
    ///    `x.write_simd(..., 1)` will overlap (assuming packet width > 1)
    fn write_simd(&mut self, value: Self::SIMD, index: usize);
}
