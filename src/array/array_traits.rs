pub trait GetWriteableBuffer {
    type Buffer;

    // unsafe fn get_buffer(&self) -> (Self::Buffer, usize);

    // unsafe fn get_buffer_checked(&self, len: usize) -> Option<Self::Buffer>;

    // unsafe fn set_buffer_no_free(&mut self);

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer>;
}
