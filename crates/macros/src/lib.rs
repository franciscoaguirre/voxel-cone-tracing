use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemStruct};

#[proc_macro]
pub fn my_macro(tokens: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(tokens as ItemStruct);
    item_struct.ident = parse_quote!(RenamedStruct);
    item_struct.vis = parse_quote!(pub);
    let output = quote! {
        #item_struct
    };
    output.into()
}
