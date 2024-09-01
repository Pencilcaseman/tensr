pub trait GetWriteableBuffer {
    type Buffer;

    /// If possible, return the underlying buffer and tell this container to
    /// not free the data when it is dropped. This enables data reuse in the
    /// lazy evaluated function system.
    ///
    /// If the buffer is too small (i.e. smaller than `len`), we return
    /// `None`. This is to ensure buffer reuse is safe and valid.
    ///
    /// If the buffer cannot be safely returned, we return `None`. Again,
    /// this allows recursive calls of this function in nested evaluation
    /// contexts, so we can reuse existing buffers if possible.
    ///
    /// # Safety
    /// The caller MUST ensure that the buffer will not be used after this
    /// operation, and must also ensure that the buffer is freed elsewhere.
    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer>;
}
