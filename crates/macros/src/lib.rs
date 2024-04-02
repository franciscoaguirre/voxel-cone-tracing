use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Item};

mod kernel;
mod sub_menus;

/// Expects the following to be in scope:
/// - `SubMenu` trait
/// - `egui`
/// - `Ui`
/// - `get_button_text`
#[proc_macro_derive(SimplifySubMenus)]
pub fn simplify_sub_menus(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sub_menus::simplify_sub_menus_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// To use on an enum where each variant is the name
/// of a type that implements the `Kernel` trait.
/// It will implement `Kernel` for that enum in a way
/// that it's `update` will call all variant's `update` functions.
#[proc_macro_attribute]
pub fn aggregated_kernel(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    kernel::aggregated_kernel_inner(args.into(), input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
