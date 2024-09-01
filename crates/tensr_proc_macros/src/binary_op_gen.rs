use itertools::Itertools;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr,
};

/// Reformat a token stream for better error messages and debugging
///
/// # Arguments
/// * `tokens` - The token stream to reformat
///
/// # Returns
/// * The reformatted token stream
///
/// # Panics
/// * If the token stream cannot be parsed
/// * If the token stream is not valid rust code
#[track_caller]
fn pretty_print(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let stream = match syn::parse_str::<syn::File>(&tokens.to_string()) {
        Ok(f) => f,
        Err(e) => {
            panic!("Pretty Print Error: {:?}", e);
        }
    };

    let pretty = prettyplease::unparse(&stream);

    let parsed = match syn::parse_str::<syn::File>(&pretty) {
        Ok(f) => f,
        Err(e) => {
            panic!("Pretty Print Error: {:?}", e);
        }
    };

    parsed.to_token_stream()
}

/// A struct representing the available binary operations
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOperation {
    /// Return the typename of the binary operation. This is the
    /// name of the standard library [`std::ops`] trait.
    pub fn type_name(&self) -> proc_macro2::TokenStream {
        match self {
            BinaryOperation::Add => quote::quote! { Add },
            BinaryOperation::Sub => quote::quote! { Sub },
            BinaryOperation::Mul => quote::quote! { Mul },
            BinaryOperation::Div => quote::quote! { Div },
        }
    }

    /// Returns the function name of the binary operation. This is
    /// the function call in the standard library [`std::ops`] trait.
    pub fn op_name(&self) -> proc_macro2::TokenStream {
        match self {
            BinaryOperation::Add => quote::quote! { add },
            BinaryOperation::Sub => quote::quote! { sub },
            BinaryOperation::Mul => quote::quote! { mul },
            BinaryOperation::Div => quote::quote! { div },
        }
    }

    /// Returns the name of the kernel for the binary operation. The
    /// kernel name is part of the [`Backend`] trait used in the main
    /// [`tensr`] crate.
    pub fn kernel_name(&self) -> proc_macro2::TokenStream {
        match self {
            BinaryOperation::Add => quote::quote! { AddKernel },
            BinaryOperation::Sub => quote::quote! { SubKernel },
            BinaryOperation::Mul => quote::quote! { MulKernel },
            BinaryOperation::Div => quote::quote! { DivKernel },
        }
    }
}

impl Parse for BinaryOperation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let op = input.parse::<syn::Ident>()?;

        match op.to_string().as_ref() {
            "Add" => Ok(BinaryOperation::Add),
            "Sub" => Ok(BinaryOperation::Sub),
            "Mul" => Ok(BinaryOperation::Mul),
            "Div" => Ok(BinaryOperation::Div),
            _ => Err(syn::Error::new(op.span(), "Invalid operation")),
        }
    }
}

/// Enum representing the reference state of a variable.
///
/// An [`Own'] variable is owned by the caller.
/// A [`Ref`] variable is borrowed from the caller.
/// A [`RefMut`] variable is mutably borrowed from the caller.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RefType {
    Own,
    Ref,
    RefMut,
}

impl ToTokens for RefType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        *tokens = match self {
            RefType::Own => quote::quote! { Own },
            RefType::Ref => quote::quote! { Ref },
            RefType::RefMut => quote::quote! { RefMut },
        }
    }
}

impl Parse for RefType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ref_type = input.parse::<syn::Ident>()?;

        match ref_type.to_string().as_ref() {
            "Own" => Ok(RefType::Own),
            "Ref" => Ok(RefType::Ref),
            "RefMut" => Ok(RefType::RefMut),
            _ => {
                Err(syn::Error::new(ref_type.span(), "Invalid reference type"))
            }
        }
    }
}

/// Enum representing the type of argument a function takes.
///
/// An [`ArrayBase`] argument is a Tensr array.
/// A [`TensrFn2`] argument is a binary Tensr function.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ArgumentType {
    ArrayBase,
    TensrFn2,
}

impl Parse for ArgumentType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let arg_type = input.parse::<syn::Ident>()?;

        match arg_type.to_string().as_ref() {
            "ArrayBase" => Ok(ArgumentType::ArrayBase),
            "TensrFn2" => Ok(ArgumentType::TensrFn2),
            _ => Err(syn::Error::new(arg_type.span(), "Invalid argument type")),
        }
    }
}

impl ToTokens for ArgumentType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        *tokens = match self {
            ArgumentType::ArrayBase => quote::quote! { ArrayBase },
            ArgumentType::TensrFn2 => quote::quote! { TensrFn2 },
        }
    }
}

/// A struct used to process macro arguments. It stores a [`RefType`]
/// and an [`ArgumentType`] and represents a possible argument to
/// a Tensr function.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Argument {
    pub ref_type: RefType,
    pub arg_type: ArgumentType,
}

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tuple = if let Expr::Tuple(tuple) = input.parse()? {
            tuple
        } else {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "Expected tuple with two elements. Received {:?}",
                    input
                ),
            ));
        };

        let first = tuple.elems[0].to_token_stream();
        let second = tuple.elems[1].to_token_stream();

        let ref_type: RefType = syn::parse2(first)?;
        let arg_type: ArgumentType = syn::parse2(second)?;

        Ok(Argument { ref_type, arg_type })
    }
}

/// Implements a binary operation for a given pair of arguments. The return
/// type is a lazily evaluated function object.
///
/// # Arguments
/// * `op_type` - The binary operation to implement
/// * `arguments` - The argument types for the function
///
/// # Returns
/// * The token stream for the trait implementation
pub fn gen_binary_op(
    op_type: &BinaryOperation,
    arguments: &[Argument],
) -> proc_macro2::TokenStream {
    /// TODO: Refactor into a method of the argument struct?
    /// Generate the generic types for a given input argument type.
    ///
    /// # Types
    /// ## `ArrayBase`
    /// * `StorageType${name}`
    /// * `NDims${name}`
    ///
    /// ## `TensrFn2`
    /// * `Op${name}`
    /// * `LhsType${name}`
    /// * `RhsType${name}`
    ///
    /// # Arguments
    /// * `arg` - The type to generate the generics for
    /// * `name` - The name variant of the argument
    ///
    /// # Returns
    /// * The token stream for the generics
    fn gen_generics(arg: &Argument, name: &str) -> proc_macro2::TokenStream {
        match arg.arg_type {
            ArgumentType::ArrayBase => {
                let storage_type: syn::Type =
                    syn::parse_str(&format!("StorageType{}", name)).unwrap();

                let ndims_type: syn::Type =
                    syn::parse_str(&format!("NDims{}", name)).unwrap();

                quote::quote! { #storage_type, #ndims_type }
            }
            ArgumentType::TensrFn2 => {
                let op_type: syn::Type =
                    syn::parse_str(&format!("Op{}", name)).unwrap();

                let lhs_type: syn::Type =
                    syn::parse_str(&format!("LhsType{}", name)).unwrap();

                let rhs_type: syn::Type =
                    syn::parse_str(&format!("RhsType{}", name)).unwrap();

                quote::quote! { #op_type, #lhs_type, #rhs_type }
            }
        }
    }

    /// Generate the generic trait bounds for a given input type.
    ///
    /// # Types
    /// ## `ArrayBase`
    /// * `StorageType${name}: traits::Storage`
    /// * `NDims${name}: Dimension`
    ///
    /// ## `TensrFn2`
    /// * `Op${name}: op_traits::BinaryOp`
    /// * `LhsType${name}: GetWriteableBuffer`
    /// * `RhsType${name}: GetWriteableBuffer<Buffer = LhsType${name}::Buffer>`
    ///
    /// # Arguments
    /// * `arg` - The type to generate the generic bounds for
    /// * `name` - The name variant of the argument
    ///
    /// # Returns
    /// * The token stream for the generic bounds
    fn gen_generic_bounds(
        arg: &Argument,
        name: &str,
    ) -> proc_macro2::TokenStream {
        match arg.arg_type {
            ArgumentType::ArrayBase => {
                let storage_type: syn::Type =
                    syn::parse_str(&format!("StorageType{}", name)).unwrap();

                let ndims_type: syn::Type =
                    syn::parse_str(&format!("NDims{}", name)).unwrap();

                quote::quote! {
                    #storage_type: traits::Storage,
                    #ndims_type: Dimension,
                }
            }
            ArgumentType::TensrFn2 => {
                let op_type: syn::Type =
                    syn::parse_str(&format!("Op{}", name)).unwrap();

                let lhs_type: syn::Type =
                    syn::parse_str(&format!("LhsType{}", name)).unwrap();

                let rhs_type: syn::Type =
                    syn::parse_str(&format!("RhsType{}", name)).unwrap();

                quote::quote! {
                    #op_type: op_traits::BinaryOp,
                    #lhs_type: GetWriteableBuffer,
                    #rhs_type: GetWriteableBuffer<Buffer = #lhs_type::Buffer>,
                }
            }
        }
    }

    /// Generate the reference type for a given argument. For an owned type,
    /// this is empty. For a reference, a lifetime is required, giving `&'a`.
    /// A mutable reference also requires a lifetime, giving `&'a mut`.
    fn gen_ref_type(arg: &Argument) -> proc_macro2::TokenStream {
        match arg.ref_type {
            RefType::Own => quote::quote! {},
            RefType::Ref => quote::quote! { &'a },
            RefType::RefMut => quote::quote! { &'a mut },
        }
    }

    /// Generate the whole type for a given argument. This is a complete type
    /// with reference identifier, lifetime and generics.
    fn gen_type(
        arg: &Argument,
        ref_type: &proc_macro2::TokenStream,
        generic: &proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        match arg.arg_type {
            ArgumentType::ArrayBase => {
                quote::quote! { #ref_type ArrayBase<Backend, #generic> }
            }
            ArgumentType::TensrFn2 => {
                quote::quote! { #ref_type TensrFn2<'a, Backend, #generic> }
            }
        }
    }

    // True if the function definition requres a lifetime generic. This is not
    // the cleanest code, but I can't think of a better way to do it that
    // doesn't make things more complicated.
    let requires_lifetime = arguments.iter().any(|a| {
        a.ref_type == RefType::Ref
            || a.ref_type == RefType::RefMut
            || a.arg_type == ArgumentType::TensrFn2
    });

    // If one of the arguments requires a lifetime, we use 'a. If not, we don't
    // need anything here. We add a comma to simplify the usage of this
    // value. This allows us to write something like:
    // <#lifetime_generic_comma Backend, ...>
    let lifetime_generic_comma = if requires_lifetime {
        quote::quote! { 'a, }
    } else {
        quote::quote! {}
    };

    // A function object requires a lifetime. We use 'a if one if the inputs
    // requires a lifetime, otherwise we use 'static.
    let output_lifetime = if requires_lifetime {
        quote::quote! { 'a }
    } else {
        quote::quote! { 'static }
    };

    let lhs_generics = gen_generics(&arguments[0], "Lhs");
    let rhs_generics = gen_generics(&arguments[1], "Rhs");

    let lhs_generic_bounds = gen_generic_bounds(&arguments[0], "Lhs");
    let rhs_generic_bounds = gen_generic_bounds(&arguments[1], "Rhs");

    let lhs_ref_type = gen_ref_type(&arguments[0]);
    let rhs_ref_type = gen_ref_type(&arguments[1]);

    let lhs_type = gen_type(&arguments[0], &lhs_ref_type, &lhs_generics);
    let rhs_type = gen_type(&arguments[1], &rhs_ref_type, &rhs_generics);

    let op_type_name = op_type.type_name();
    let op_name = op_type.op_name();
    let kernel_name = op_type.kernel_name();

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

            #[inline(always)]
            fn #op_name(
                self,
                rhs: #rhs_type,
            ) -> Self::Output {
                Self::Output::new(self, rhs)
            }
        }
    };

    pretty_print(result)
}

/// Generate all possible combinations from a pair of argument types,
/// including owned, reference and mutable reference types.
pub fn gen_type_pairs(
    types: [ArgumentType; 2],
) -> Vec<((RefType, ArgumentType), (RefType, ArgumentType))> {
    // f(["A", "B"]) => [
    //      [(Own, "A"), (Own, "A")],
    //      [(Own, "A"), (Ref, "A")],
    //      [(Ref, "A"), (Own, "A")],
    //      [(Ref, "A"), (Ref, "A")],
    //
    //      [(Own, "A"), (Own, "B")],
    //      [(Own, "A"), (Ref, "B")],
    //      [(Ref, "A"), (Own, "B")],
    //      [(Ref, "A"), (Ref, "B")],
    //
    //      [(Own, "B"), (Own, "A")],
    //      [(Own, "B"), (Ref, "A")],
    //      [(Ref, "B"), (Own, "A")],
    //      [(Ref, "B"), (Ref, "A")],
    //
    //      [(Own, "B"), (Own, "B")],
    //      [(Own, "B"), (Ref, "B")],
    //      [(Ref, "B"), (Own, "B")],
    //      [(Ref, "B"), (Ref, "B")],
    // ]

    let new_types: Vec<_> = types
        .iter()
        .flat_map(|t| {
            [(RefType::Own, *t), (RefType::Ref, *t), (RefType::RefMut, *t)]
        })
        .collect();

    // Cartesian product with itself to get all valid combinations
    new_types.clone().into_iter().cartesian_product(new_types).collect()
}

/// Generate a binary operation implementation for a pair of argumets.
pub fn gen(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as Expr);

    let tuple = if let Expr::Tuple(tuple) = &input {
        tuple
    } else {
        panic!(
            "Expected tuple with two elements. Received {:?}",
            input.to_token_stream().to_string()
        );
    };

    let stream = tuple.elems[0].to_token_stream().into();
    let op_type = parse_macro_input!(stream as BinaryOperation);

    let array = if let Expr::Array(arr) = &tuple.elems[1] {
        arr
    } else {
        panic!(
            "Expected tuple with two elements. Received {:?}",
            input.to_token_stream().to_string(),
        );
    };

    // array.elems.iter() doesn't return a normal iterator, so I can't rewrite
    // this as a map for some reason.
    let mut arguments = Vec::new();
    for elem in array.elems.iter() {
        let stream = elem.to_token_stream().into();
        let t = parse_macro_input!(stream as Argument);
        arguments.push(t);
    }

    if arguments.len() != 2 {
        panic!(
            "Expected two arguments. Received {:?}",
            input.to_token_stream().to_string()
        );
    }

    let result = gen_binary_op(&op_type, &arguments);
    pretty_print(result).into()
}

/// Generate all possible implementations of a binary operation for the
/// available input types.
pub fn gen_all(tok: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tok as Expr);

    let stream = input.to_token_stream().into();
    let op = parse_macro_input!(stream as BinaryOperation);

    let perms =
        gen_type_pairs([ArgumentType::ArrayBase, ArgumentType::TensrFn2]);

    let mut result = String::new();

    for ((lhs_ref, lhs_type), (rhs_ref, rhs_type)) in perms {
        let expr = format!(
            r#"
            tensr_proc_macros::generate_binary_op!((
                {:?},
                [
                    (
                        {lhs_ref:?},
                        {lhs_type:?}
                    ),
                    (
                        {rhs_ref:?},
                        {rhs_type:?}
                    )
                ]
            ));
        "#,
            op,
        );
        result.push_str(&format!("{expr}\n"));
    }

    let stream: syn::File = syn::parse_str(&result).unwrap();
    pretty_print(stream.to_token_stream()).into()
}
