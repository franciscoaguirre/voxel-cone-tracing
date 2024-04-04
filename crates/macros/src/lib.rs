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
/// of a type that implements the `Kernel` and `Pausable` traits.
/// It will implement `Kernel` for that enum in a way
/// that it's `update` will call all variant's `update` functions,
/// and respect pausing.
/// Meant to be used as the top-level `T: Kernel` required for
/// `RenderLoop`.
/// The resulting `Kernel` can't be paused.
#[proc_macro_attribute]
pub fn aggregated_kernel(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    kernel::aggregated_kernel_inner(args.into(), input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Allows grouping a bunch of `Kernel`s together in a struct.
/// Setup and update will be called in the order of the fields defined in the struct.
/// This group works as a unit and is paused together.
#[proc_macro_attribute]
pub fn kernel_group(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    kernel::kernel_group_inner(args.into(), input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// For help creating a `Kernel` implementation.
/// Only adds the pause functionality.
// TODO: Could support an argument: `can_pause: bool`.
#[proc_macro_attribute]
pub fn pausable(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    kernel::pausable_inner(args.into(), input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
