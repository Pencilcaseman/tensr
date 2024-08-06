mod binary_op_gen;
mod function_gen;

use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_function_type(tok: TokenStream) -> TokenStream {
    function_gen::function_gen(tok)
}

#[proc_macro]
pub fn generate_binary_op(tok: TokenStream) -> TokenStream {
    binary_op_gen::gen(tok)
}
