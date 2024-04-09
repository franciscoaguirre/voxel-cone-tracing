use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod sub_menus;
mod system;

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

/// To use on an enum or a struct that only holds types that implement
/// `System`.
///
/// If used on an enum, it will implement `System` on it such that
/// all methods are forwarded to the corresponding variant.
///
/// If used on a struct, it will effectively create a group, where
/// all methods are run sequentially in the order specified by the
/// variants.
///
/// All methods generated respect pausing.
#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    system::derive_system_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// For help creating a `System` implementation.
/// Only adds the pause functionality.
/// Requires an attribute `paused: bool`.
// TODO: Could support an argument: `can_pause: bool`.
#[proc_macro_derive(Pausable)]
pub fn derive_pausable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    system::derive_pausable_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Only for enums.
/// Creates a `SubMenu` implementation where
/// all methods are forwarded to the correct variant.
#[proc_macro_derive(SubMenu)]
pub fn derive_submenu(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sub_menus::derive_submenu_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Basically the same as `derive_pausable` but with `should_show`
/// instead of `paused`.
/// Meant to be used with submenus instead of systems.
#[proc_macro_derive(Showable)]
pub fn derive_showable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    sub_menus::derive_showable_inner(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
