use crate::array::array_traits::GetWriteableBuffer;
use crate::backend::op_traits;
use crate::backend::traits;
use std::marker::PhantomData;
use tensr_proc_macros::generate_function_type;

pub trait Function2<Out> {
    fn apply(&self, out: &mut Out);
}

generate_function_type!([Ref, Ref]);
generate_function_type!([Own, Ref]);
generate_function_type!([Own, Own]);
generate_function_type!([Ref, Own]);

impl<'a, Backend, Op, Arg0, Arg1> GetWriteableBuffer
    for Function2OwnRef<'a, Backend, Op, Arg0, Arg1>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Arg0: traits::LazyArrayObject + GetWriteableBuffer,
    Arg1: traits::LazyArrayObject,
{
    type Buffer = Arg0::Buffer;

    // unsafe fn get_buffer(&self) -> (Self::Buffer, usize) {
    //     self.arg0.get_buffer()
    // }
    //
    // unsafe fn get_buffer_checked(&self, len: usize) -> Option<Self::Buffer> {
    //     self.arg0.get_buffer_checked(len)
    // }
    //
    // unsafe fn set_buffer_no_free(&mut self) {
    //     self.arg0.set_buffer_no_free();
    // }

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        self.arg0.get_buffer_and_set_no_free(len)
    }
}

impl<Backend, Op, Arg0, Arg1> GetWriteableBuffer
    for Function2OwnOwn<Backend, Op, Arg0, Arg1>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Arg0: traits::LazyArrayObject + GetWriteableBuffer,
    Arg1: traits::LazyArrayObject + GetWriteableBuffer<Buffer = Arg0::Buffer>,
{
    type Buffer = Arg0::Buffer;

    // unsafe fn get_buffer(&self) -> (Self::Buffer, usize) {
    //     self.arg0.get_buffer()
    // }
    //
    // unsafe fn get_buffer_checked(&self, len: usize) -> Option<Self::Buffer> {
    //     self.arg0.get_buffer_checked(len)
    // }
    //
    // unsafe fn set_buffer_no_free(&mut self) {
    //     self.arg0.set_buffer_no_free();
    // }

    unsafe fn get_buffer_and_set_no_free(
        &mut self,
        len: usize,
    ) -> Option<Self::Buffer> {
        self.arg0
            .get_buffer_and_set_no_free(len)
            .or(self.arg1.get_buffer_and_set_no_free(len))
    }
}
