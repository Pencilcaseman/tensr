use crate::array::array_traits::HasWriteableBuffer;
use crate::backend::op_traits;
use crate::backend::traits;
use crate::types;
use std::marker::PhantomData;
use tensr_proc_macros::generate_function_type;

pub trait Function2<Out> {
    fn apply(&self, out: &mut Out);
}

generate_function_type!([Ref, Ref]);
generate_function_type!([Own, Ref]);

unsafe impl<'a, Backend, Op, Arg0, Arg1> HasWriteableBuffer
    for Function2OwnRef<'a, Backend, Op, Arg0, Arg1>
where
    Backend: traits::Backend,
    Op: op_traits::BinaryOp,
    Arg0: traits::LazyArrayObject + HasWriteableBuffer,
    Arg1: traits::LazyArrayObject,
{
    type Buffer = Arg0::Buffer;

    unsafe fn get_buffer(&self) -> (Self::Buffer, usize) {
        self.arg0.get_buffer()
    }

    unsafe fn set_buffer_no_free(&mut self) {
        self.arg0.set_buffer_no_free();
    }
}
