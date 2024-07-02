use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{bracketed, parse_macro_input, DeriveInput, Expr, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ownership {
    Ref,
    Own,
}

impl Parse for Ownership {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ownership = input.parse::<syn::Ident>()?;

        match ownership.to_string().as_str() {
            "Ref" => Ok(Ownership::Ref),
            "Own" => Ok(Ownership::Own),
            _ => Err(syn::Error::new(
                ownership.span(),
                "Invalid ownership specifier",
            )),
        }
    }
}

#[proc_macro]
pub fn generate_function_type(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as Expr);

    let mut owner_types = Vec::new();
    if let Expr::Array(arr) = input {
        for elem in arr.elems.into_iter() {
            let stream = elem.to_token_stream().into();
            let ownership = parse_macro_input!(stream as Ownership);
            owner_types.push(ownership);
        }
    }

    let trait_types = owner_types
        .iter()
        .enumerate()
        .map(|(i, v)| {
            // let name = format_ident!("Arg{}", i);
            // match v {
            //     Ownership::Ref => quote::quote! { &'a #name },
            //     Ownership::Own => quote::quote! { #name },
            // }
            let name = format_ident!("Arg{i}");
            quote::quote! { #name }
        })
        .collect::<Vec<_>>();

    let trait_types = if owner_types.iter().any(|t| t == &Ownership::Ref) {
        // Type requires a lifetime parameter
        quote::quote! {
            <'a, Backend, Op, #(#trait_types),*>
        }
    } else {
        quote::quote! {
            <Backend, Op, #(#trait_types),*>
        }
    };

    let mut type_name = format!("Function{}", owner_types.len());
    for v in owner_types.iter() {
        match v {
            Ownership::Ref => type_name.push_str("Ref"),
            Ownership::Own => type_name.push_str("Own"),
        }
    }
    let type_name = format_ident!("{}", type_name);

    let mut trait_bounds = String::new();
    trait_bounds.push_str("where Backend: traits::Backend, ");
    trait_bounds.push_str("Op: op_traits::BinaryOp, ");
    for (i, _) in owner_types.iter().enumerate() {
        trait_bounds.push_str(&format!("Arg{i}: traits::LazyArrayObject, "));
    }

    let trait_bounds = syn::parse_str::<syn::WhereClause>(&trait_bounds)
        .map_err(|e| e.to_compile_error())
        .unwrap();

    let out = quote::quote! {
        pub struct #type_name #trait_types
        #trait_bounds
        {

        }
    };

    panic!("{}", out.to_string());

    // out.into()
}
