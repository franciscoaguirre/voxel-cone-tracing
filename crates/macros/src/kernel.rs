use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Error, Item, Result};

pub(crate) fn aggregated_kernel_inner(_args: TokenStream2, input: Item) -> Result<TokenStream2> {
    let data_enum = match &input {
        Item::Enum(data_enum) => data_enum,
        _ => return Err(Error::new_spanned(&input, "Expected an enum")),
    };

    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    // for variant in data_enum.variants.iter() {
    //     let kernel_ident = &variant.ident;
    // }

    let aggregated_enum_ident = &data_enum.ident;
    let aggregated_enum = quote! {
        enum #aggregated_enum_ident {
            #(#variant_idents(#variant_idents)),*
        }
    };
    let aggregated_enum_impl = quote! {
        impl Kernel for #aggregated_enum_ident {
            unsafe fn setup(&mut self) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => inner_kernel.setup()),*
                }
            }
            unsafe fn update(&mut self, scene: &Scene) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => inner_kernel.update(scene)),*
                }
            }
        }
    };

    Ok(quote! {
        #aggregated_enum
        #aggregated_enum_impl
    })
}
