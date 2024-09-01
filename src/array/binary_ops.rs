use crate::{
    array::{
        base::ArrayBase, function_2::TensrFn2, traits::GetWriteableBuffer,
    },
    backend::{op_traits, traits},
    dimension::dim::Dimension,
};

tensr_proc_macros::generate_all_binary_ops!(Add);
tensr_proc_macros::generate_all_binary_ops!(Sub);
tensr_proc_macros::generate_all_binary_ops!(Mul);
tensr_proc_macros::generate_all_binary_ops!(Div);
