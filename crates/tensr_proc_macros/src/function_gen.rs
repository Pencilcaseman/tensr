use proc_macro::TokenStream;
use quote::{format_ident, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr};

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

pub fn function_gen(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as Expr);

    // INFO: [ Ownership::Ref, Ownership::Own, ... ]
    let mut owner_types = Vec::new();
    if let Expr::Array(arr) = input {
        for elem in arr.elems.into_iter() {
            let stream = elem.to_token_stream().into();
            let ownership = parse_macro_input!(stream as Ownership);
            owner_types.push(ownership);
        }
    }

    let fn_dimensions = owner_types.len();

    // INFO: [ "Arg0", "Arg1", ... ]
    let trait_types = owner_types
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let name = format_ident!("Arg{i}");
            quote::quote! { #name }
        })
        .collect::<Vec<_>>();

    // INFO: <'a, Backend, Op, Arg0, Arg1, ...>
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

    // INFO: Function2RefRef or Function2OwnOwn or ...
    let mut type_name = format!("Function{}", owner_types.len());
    for v in owner_types.iter() {
        match v {
            Ownership::Ref => type_name.push_str("Ref"),
            Ownership::Own => type_name.push_str("Own"),
        }
    }
    let type_name = format_ident!("{}", type_name);

    // INFO: Trait bounds (including "where")
    let mut trait_bounds = String::new();
    trait_bounds.push_str("where Backend: traits::Backend, ");

    // TODO: Make binary op more generic
    trait_bounds.push_str("Op: op_traits::BinaryOp, ");
    for (i, _) in owner_types.iter().enumerate() {
        trait_bounds.push_str(&format!("Arg{i}: traits::LazyArrayObject, "));
    }

    // INFO: Trait bounds (parsed)
    let trait_bounds = syn::parse_str::<syn::WhereClause>(&trait_bounds)
        .map_err(|e| e.to_compile_error())
        .unwrap();

    // INFO: arg0: Arg0, arg1: Arg1, ...
    let argument_attributes: Vec<_> = owner_types
        .iter()
        .enumerate()
        .map(|(n, o)| {
            let name = format_ident!("arg{}", n);
            let dtype = format_ident!("Arg{}", n);
            match o {
                Ownership::Ref => {
                    quote::quote! { #name: &'a #dtype }
                }
                Ownership::Own => quote::quote! { #name: #dtype },
            }
        })
        .collect();

    // INFO: arg0: &'a Arg0, arg1: &'a Arg1, ...
    let arg_params: Vec<_> = owner_types
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let name = format_ident!("arg{}", i);
            let dtype = format_ident!("Arg{}", i);
            match v {
                Ownership::Ref => quote::quote! { #name: &'a #dtype },
                Ownership::Own => quote::quote! { #name: #dtype },
            }
        })
        .collect();

    let args: Vec<_> = (0..fn_dimensions)
        .map(|n| {
            let name = format_ident!("arg{}", n);
            quote::quote! { #name }
        })
        .collect();

    // INFO: Struct definition
    let out = quote::quote! {
        pub struct #type_name #trait_types // Type name and trait types
            #trait_bounds                  // Trait bounds
        {
            #(pub(crate) #argument_attributes,)*
            pub(crate) backend: PhantomData<Backend>,
            pub(crate) op: PhantomData<Op>,
        }

        impl #trait_types traits::ContainerLength for #type_name #trait_types
            #trait_bounds
        {
            fn len(&self) -> usize {
                self.arg0.len()
            }
        }

        impl #trait_types traits::ContainerScalarType for #type_name #trait_types
            #trait_bounds
        {
            type Scalar = Arg0::Scalar;
        }

        impl #trait_types traits::ContainerStorageType for #type_name #trait_types
            #trait_bounds
        {
            type Storage = Arg0::Storage;
        }

        impl #trait_types traits::ContainerBackendType for #type_name #trait_types
            #trait_bounds
        {
            type Backend = Backend;
        }

        impl #trait_types traits::LazyArrayObject for #type_name #trait_types
            #trait_bounds
        {
        }

        impl #trait_types #type_name #trait_types
            #trait_bounds
        {
            #[inline(always)]
            pub fn new( #(#arg_params),* ) -> Self
                #trait_bounds
            {
                Self { #(#args),*, backend: PhantomData, op: PhantomData }
            }
        }

        // unsafe impl #trait_types HasWriteableBuffer
        // for #type_name #trait_types
        //     #trait_bounds
        // {
        //     type Buffer = Arg0::Buffer;
        //
        //     unsafe fn get_buffer(&self) -> (Self::Buffer, usize) {
        //         self.arg0.get_buffer()
        //     }
        //
        //     unsafe fn get_buffer_checked(&self, len: usize) -> Option<Self::Buffer> {
        //         self.arg0.get_buffer_checked(len)
        //     }
        //
        //     unsafe fn set_buffer_no_free(&mut self) {
        //         self.arg0.set_buffer_no_free();
        //     }
        // }
    };

    // We pretty-print the output for better error messages and debugging
    let stream = syn::parse_file(&out.to_string()).unwrap();
    stream.to_token_stream().into()
}
