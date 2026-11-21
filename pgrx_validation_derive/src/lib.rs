#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned};

/// Valid prefixes for the validation functions.
const VALID_PREFIXES: [&str; 4] =
    ["must_be_", "must_not_be_", "must_contain_", "must_not_contain_"];

#[proc_macro_attribute]
/// Checks the validation function against the rules of this crate and, with
/// the `pgrx` feature, exposes it to PostgreSQL.
///
/// # Implementation details
///
/// The [`validation`] attribute macro is meant to be used on functions that
/// return a `Result<(), crate::Error>`, where `crate::Error` is the error type
/// of the crate defining the validation.
///
/// The function itself is always emitted unchanged. When the feature flag
/// `pgrx` is enabled, the macro also emits a hidden `<name>_pg_extern`
/// function, registered with `pg_extern` under the SQL name of the original
/// function, which returns `true` when the validation passes. When the
/// validation fails, it raises a PostgreSQL error carrying the `Display` text
/// of the error through the `pgrx::error!` macro.
pub fn validation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    // Extract parts of the function signature
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let fn_name = &sig.ident;

    // Methods wrapped with a `validation` attribute must be `pub` to be exposed
    // as a pg_extern.
    if !matches!(vis, syn::Visibility::Public(_)) {
        let error_message = format!(
            "Function `{fn_name}` must be public to be decorated with `validation` - add a `pub` keyword before the function definition if appropriate.",
        );
        return syn::Error::new(vis.span(), &error_message).to_compile_error().into();
    }

    // Methods wrapped with a `validation` attribute must not be async.
    if sig.asyncness.is_some() {
        let error_message =
            format!("Function `{fn_name}` must not be async to be decorated with `validation`.");
        return syn::Error::new(sig.asyncness.span(), &error_message).to_compile_error().into();
    }

    // Since Postgres does not support generics, we need to ensure that the
    // function does not have any.
    if !sig.generics.params.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must not have any generic parameters to be decorated with `validation`.",
        );
        return syn::Error::new(sig.generics.span(), &error_message).to_compile_error().into();
    }

    // A validation only makes sense when the function receives one or more
    // arguments.
    if sig.inputs.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must have at least one argument to be decorated with `validation`.",
        );
        return syn::Error::new(sig.inputs.span(), &error_message).to_compile_error().into();
    }

    // We expect the function to return a `Result<(), crate::Error>`.
    if !is_result_unit_crate_error(&sig.output) {
        let error_message = format!(
            "Function `{fn_name}` must return a `Result<(), crate::Error>` to be decorated with `validation`, but returns `{output}`.",
            output = match &sig.output {
                syn::ReturnType::Type(_, ty) => quote!(#ty).to_string(),
                syn::ReturnType::Default => "()".to_string(),
            },
        );
        return syn::Error::new(sig.output.span(), &error_message).to_compile_error().into();
    }

    // We expect the function name to be snake_case.
    let function_name = fn_name.to_string();
    if !function_name.chars().all(|character: char| {
        character.is_ascii_lowercase() || character.is_numeric() || character == '_'
    }) {
        let error_message =
            format!("Function `{fn_name}` must be snake_case to be decorated with `validation`.");
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // We expect the function name to start with `must_be_` or `must_not_be_`.
    if !VALID_PREFIXES.iter().any(|prefix| function_name.starts_with(prefix)) {
        let error_message = format!(
            "Function `{fn_name}` must start with any of {VALID_PREFIXES:?} to be decorated with `validation`.",
        );
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // We expect that a `validation` method has a doc comment.
    if input_fn.attrs.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must have a doc comment to be decorated with `validation`.",
        );
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // Without the pgrx feature, the function is left unchanged.
    if !cfg!(feature = "pgrx") {
        return quote!(#input_fn).into();
    }

    let inputs = &sig.inputs;

    // Collect argument names to be passed along when calling the validation.
    let arg_names = inputs.iter().map(|arg| {
        match arg {
            syn::FnArg::Receiver(_) => quote! { self },
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                quote! { #pat }
            }
        }
    });

    // The wrapper exposed to PostgreSQL under the name of the validation,
    // which returns `true` or raises the validation error.
    let sql_name = fn_name.to_string();
    let wrapper_fn_name = syn::Ident::new(&format!("{fn_name}_pg_extern"), fn_name.span());
    quote! {
        #input_fn

        #[doc(hidden)]
        #[pgrx::pg_extern(name = #sql_name)]
        pub fn #wrapper_fn_name (#inputs) -> bool {
            match #fn_name(#(#arg_names),*) {
                Ok(()) => true,
                Err(error) => {
                    pgrx::error!("Validation failed: {}", error);
                }
            }
        }
    }
    .into()
}

/// Determines if the return type is `Result<(), crate::Error>`.
fn is_result_unit_crate_error(output: &syn::ReturnType) -> bool {
    let syn::ReturnType::Type(_, ty) = output else {
        return false;
    };
    let syn::Type::Path(type_path) = &**ty else {
        return false;
    };
    let Some(last_segment) = type_path.path.segments.last() else {
        return false;
    };
    let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        return false;
    };
    let mut arguments = args.args.iter();
    let (
        Some(syn::GenericArgument::Type(syn::Type::Tuple(ok_type))),
        Some(syn::GenericArgument::Type(syn::Type::Path(error_path))),
        None,
    ) = (arguments.next(), arguments.next(), arguments.next())
    else {
        return false;
    };
    last_segment.ident == "Result" && ok_type.elems.is_empty() && is_crate_error(error_path)
}

/// Checks if a `TypePath` is `crate::Error`.
fn is_crate_error(path: &syn::TypePath) -> bool {
    path.qself.is_none()
        && path.path.leading_colon.is_none()
        && path.path.segments.len() == 2
        && path.path.segments[0].ident == "crate"
        && path.path.segments[1].ident == "Error"
        && path.path.segments.iter().all(|segment| segment.arguments.is_none())
}
