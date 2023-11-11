use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

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
