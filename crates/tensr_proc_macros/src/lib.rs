mod binary_op_gen;
mod function_gen;

use proc_macro::TokenStream;

/// Generate a binary operation implementation for a pair of argumets.
#[proc_macro]
pub fn generate_binary_op(tok: TokenStream) -> TokenStream {
    binary_op_gen::gen(tok)
}

/// Generate all possible implementations of a binary operation for the
/// available input types.
#[proc_macro]
pub fn generate_all_binary_ops(tok: TokenStream) -> TokenStream {
    binary_op_gen::gen_all(tok)
}
