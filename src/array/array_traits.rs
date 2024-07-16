/// A trait for objects whose storage can be directly written to without causing
/// problems. This trait allows operations like `a + &b` to use the underlying data
/// from `a` to store the result, meaning there are no allocations, copies or frees.
pub unsafe trait HasWriteableBuffer {
    /// The lowest-level pointer type to the underlying data. For example, for host
    /// arrays, this is a normal pointer. For CUDA arrays, this is a device pointer.
    type Buffer;

    /// Return a pointer to the underlying data and the length of the array
    ///
    /// # Safety
    /// The returned pointer must be valid for the lifetime of the object, and the
    /// array MUST have a length of exactly `len` elements.
    unsafe fn get_buffer(&self) -> (Self::Buffer, usize);

    /// Mark the data to not be freed when the main object is dropped. This is
    /// necessary for preventing invalid memory accesses.
    ///
    /// # Safety
    /// The data MUST be freed elsewhere, otherwise a memory leak will occur.
    unsafe fn set_buffer_no_free(&mut self);
}
