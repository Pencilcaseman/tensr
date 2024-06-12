/// The [`Backend`] trait is used to mark structs as a valid backend for
/// calculations. Operators should be implemented for a given backend, and
/// must operate on (valid) data provided by the [`Backend::OwnedStorage`]
/// type.
pub trait Backend {
    /// A type representing an object which can allocate, manage and store
    /// memory for a given [`Backend`]. Elements are of type `T`.
    type OwnedStorage<T>;
}

/// A [`Backend`] trait for data stored on the host
pub(crate) trait HostBackend: Backend {}

/// A trait marking an object as owning the data it contains. If this is the
/// case, the data must be stored contiguously and must be paired with a
/// backend.
pub(crate) trait OwnedStorage:
    std::ops::Index<usize> + std::ops::IndexMut<usize>
{
}

// /// A trait marking an object as referencing the data it contains. Referenced
// /// data is owned elsewhere in a wider scope, so cannot be freed, reallocated
// /// or moved in any way. It may also have a non-trivial stride.
// pub(crate) trait SharedStorage:
//     std::ops::Index<usize> + std::ops::IndexMut<usize>
// {
// }
