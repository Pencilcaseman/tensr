use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Expr,
};

// pub trait OpType {
//     /// Returns the name of the operator (e.g. "Add")
//     fn name(&self) -> String;
//
//     /// Returns a token stream representing the operation to perform.
//     ///
//     /// For example, for the `Add` operator with arguments `["a", "b"]`,
//     /// this would return `a + b`.
//     ///
//     /// With `["a", "b", "c"]`, this would return `a + b + c`. Note that this
//     /// may return None if the operation format is not supported.
//     fn operation(&self, arguments: &[String]) -> proc_macro2::TokenStream;
// }

static ADD: [&str; 3] = ["Add", "add", "+"];
static SUB: [&str; 3] = ["Sub", "sub", "-"];
static MUL: [&str; 3] = ["Mul", "mul", "*"];
static DIV: [&str; 3] = ["Div", "div", "/"];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RefType {
    Ref,
    Own,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgumentType {
    ArrayBase,
    TensrFn2,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Argument {
    pub ref_type: RefType,
    pub arg_type: ArgumentType,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Expr::Tuple(tuple) = input.parse()? {
            let ref_type =
                match tuple.elems[0].to_token_stream().to_string().as_ref() {
                    "Ref" => RefType::Ref,
                    "Own" => RefType::Own,
                    _ => {
                        return Err(syn::Error::new(
                            tuple.span(),
                            "Invalid reference type",
                        ))
                    }
                };

            let arg_type =
                match tuple.elems[1].to_token_stream().to_string().as_ref() {
                    "ArrayBase" => ArgumentType::ArrayBase,
                    "TensrFn2" => ArgumentType::TensrFn2,
                    _ => {
                        return Err(syn::Error::new(
                            tuple.span(),
                            "Invalid argument type",
                        ))
                    }
                };

            Ok(Argument { ref_type, arg_type })
        } else {
            Err(syn::Error::new(input.span(), "Expected tuple of two elements"))
        }
    }
}

pub fn gen_binary_op(
    op_type: &[syn::LitStr],
    arguments: &[Argument],
) -> proc_macro2::TokenStream {
    let op_type_name =
        syn::parse_str::<syn::Ident>(&op_type[0].value()).unwrap();
    let op_name = syn::parse_str::<syn::Ident>(&op_type[1].value()).unwrap();
    let kernel_name =
        syn::parse_str::<syn::Ident>(&format!("{op_type_name}Kernel")).unwrap();

    let requires_lifetime = arguments.iter().any(|a| {
        a.ref_type == RefType::Ref || a.arg_type == ArgumentType::TensrFn2
    });

    let lifetime_generic_comma = if requires_lifetime {
        quote::quote! { 'a, }
    } else {
        quote::quote! {}
    };

    let output_lifetime = if requires_lifetime {
        quote::quote! { 'a }
    } else {
        quote::quote! { 'static }
    };

    // TODO: Abstract this out into a function
    let lhs_generics = match arguments[0].arg_type {
        ArgumentType::ArrayBase => quote::quote! { StorageTypeLhs, NDimsLhs },
        ArgumentType::TensrFn2 => {
            quote::quote! { OpLhs, LhsTypeLhs, RhsTypeLhs }
        }
    };

    let rhs_generics = match arguments[1].arg_type {
        ArgumentType::ArrayBase => quote::quote! { StorageTypeRhs, NDimsRhs },
        ArgumentType::TensrFn2 => {
            quote::quote! { OpRhs, LhsTypeRhs, RhsTypeRhs }
        }
    };

    // TODO: Abstract this out into a function
    let lhs_generic_bounds = match arguments[0].arg_type {
        ArgumentType::ArrayBase => quote::quote! {
            StorageTypeLhs: traits::Storage,
            NDimsLhs: Dimension,
        },
        ArgumentType::TensrFn2 => quote::quote! {
            OpLhs: op_traits::BinaryOp,
            LhsTypeLhs: GetWriteableBuffer,
            RhsTypeLhs: GetWriteableBuffer<Buffer = LhsTypeLhs::Buffer>,
        },
    };

    let rhs_generic_bounds = match arguments[1].arg_type {
        ArgumentType::ArrayBase => quote::quote! {
            StorageTypeRhs: traits::Storage,
            NDimsRhs: Dimension,
        },
        ArgumentType::TensrFn2 => quote::quote! {
            OpRhs: op_traits::BinaryOp,
            LhsTypeRhs: GetWriteableBuffer,
            RhsTypeRhs: GetWriteableBuffer<Buffer = LhsTypeRhs::Buffer>,
        },
    };

    let lhs_ref_type = match arguments[0].ref_type {
        RefType::Own => quote::quote! {},
        RefType::Ref => quote::quote! { &'a },
    };

    let rhs_ref_type = match arguments[1].ref_type {
        RefType::Own => quote::quote! {},
        RefType::Ref => quote::quote! { &'a },
    };

    // TODO: Abstract this out into a function
    let lhs_type = match arguments[0].arg_type {
        ArgumentType::ArrayBase => {
            quote::quote! { #lhs_ref_type ArrayBase<Backend, #lhs_generics> }
        }
        ArgumentType::TensrFn2 => {
            quote::quote! { #lhs_ref_type TensrFn2<'a, Backend, #lhs_generics> }
        }
    };

    let rhs_type = match arguments[1].arg_type {
        ArgumentType::ArrayBase => {
            quote::quote! { #rhs_ref_type ArrayBase<Backend, #rhs_generics> }
        }
        ArgumentType::TensrFn2 => {
            quote::quote! { #rhs_ref_type TensrFn2<'a, Backend, #rhs_generics> }
        }
    };

    let result = quote::quote! {
        impl<#lifetime_generic_comma Backend, #lhs_generics, #rhs_generics>
            std::ops::#op_type_name<#rhs_type>
            for  #lhs_type
        where
            Backend: traits::Backend,
            #lhs_generic_bounds
            #rhs_generic_bounds
        {
            type Output = TensrFn2<
                #output_lifetime,
                Backend,
                Backend::#kernel_name,
                #lhs_type,
                #rhs_type,
            >;

            fn #op_name(
                self,
                rhs: #rhs_type,
            ) -> Self::Output {
                Self::Output::new(self, rhs)
            }
        }
    };

    result
}

pub fn gen(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as Expr);

    let mut op_type = Vec::new();
    let mut arguments = Vec::new();

    // if let Expr::Array(arr) = input {
    //     for elem in arr.elems.into_iter() {
    //         let stream = elem.to_token_stream().into();
    //         let t = parse_macro_input!(stream as Argument);
    //         arguments.push(t);
    //     }
    // }

    // (["Add", "add", "+"], [Operands])
    if let Expr::Tuple(tuple) = input {
        match &tuple.elems[0] {
            Expr::Array(arr) => {
                for elem in arr.elems.iter() {
                    let stream = elem.to_token_stream().into();
                    let t = parse_macro_input!(stream as syn::LitStr);
                    op_type.push(t);
                }
            }
            _ => {
                panic!("Expected array of arguments");
            }
        }

        match &tuple.elems[1] {
            Expr::Array(arr) => {
                for elem in arr.elems.iter() {
                    let stream = elem.to_token_stream().into();
                    let t = parse_macro_input!(stream as Argument);
                    arguments.push(t);
                }
            }
            _ => {
                panic!("Expected array of op types");
            }
        }
    } else {
        panic!("Expected tuple of two elements");
    }

    let result = match arguments.len() {
        0 => quote::quote! { todo!() },
        1 => quote::quote! { todo!() },
        2 => gen_binary_op(&op_type, &arguments),
        _ => quote::quote! { todo!() },
    };
    let result_file = syn::parse_file(&result.to_string());

    if let Err(e) = result_file {
        panic!("Error: {:?}", e);
    }

    let result = prettyplease::unparse(&result_file.unwrap());

    // Return the result as a TokenStream
    syn::parse_file(&result.to_string()).unwrap().to_token_stream().into()
}
