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

    let aggregated_enum_ident = &data_enum.ident;
    let aggregated_enum = quote! {
        enum #aggregated_enum_ident {
            #(#variant_idents(#variant_idents)),*
        }
    };
    let kernel_impl = quote! {
        impl Kernel for #aggregated_enum_ident {
            unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => {
                        if !inner_kernel.is_paused() {
                            inner_kernel.setup(assets);
                        }
                    }),*
                }
            }
            unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => {
                        if !inner_kernel.is_paused() {
                            inner_kernel.update(scene, assets);
                        }
                    }),*
                }
            }
        }
    };
    let pausable_impl = quote! {
        impl Pausable for #aggregated_enum_ident {
            fn pause(&mut self) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => inner_kernel.pause()),*
                }
            }
            fn unpause(&mut self) {
                match self {
                    #(Self::#variant_idents(inner_kernel) => inner_kernel.unpause()),*
                }
            }
            fn is_paused(&self) -> bool {
                match self {
                    #(Self::#variant_idents(inner_kernel) => inner_kernel.is_paused()),*
                }
            }
        }
    };

    Ok(quote! {
        #aggregated_enum
        #kernel_impl
        #pausable_impl
    })
}

pub(crate) fn kernel_group_inner(_args: TokenStream2, input: Item) -> Result<TokenStream2> {
    let data_struct = match &input {
        Item::Struct(data_struct) => data_struct,
        _ => return Err(Error::new_spanned(&input, "Expected struct.")),
    };
    let ident = &data_struct.ident;
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect();
    let field_types: Vec<_> = data_struct.fields.iter().map(|field| &field.ty).collect();

    let modified_struct = quote! {
        struct #ident {
            #(#field_idents: #field_types),*,
            paused: bool,
        }
    };
    let kernel_impl = quote! {
        impl Kernel for #ident {
            unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
                #(self.#field_idents.setup(assets));*;
            }
            unsafe fn update(&mut self, scene: &Scene, assets: &AssetRegistry) {
                #(self.#field_idents.update(scene, assets));*;
            }
        }
    };
    let pausable_impl = quote! {
        impl Pausable for #ident {
            fn pause(&mut self) {
                self.paused = true;
            }
            fn unpause(&mut self) {
                self.paused = false;
            }
            fn is_paused(&self) -> bool {
                self.paused
            }
        }
    };

    Ok(quote! {
        #modified_struct
        #kernel_impl
        #pausable_impl
    })
}

pub(crate) fn pausable_inner(_args: TokenStream2, input: Item) -> Result<TokenStream2> {
    let data_struct = match &input {
        Item::Struct(data_struct) => data_struct,
        _ => return Err(Error::new_spanned(&input, "Expected struct.")),
    };
    let ident = &data_struct.ident;
    let visibility = &data_struct.vis;
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect();
    let field_types: Vec<_> = data_struct.fields.iter().map(|field| &field.ty).collect();

    let modified_struct = quote! {
        #visibility struct #ident {
            #(#field_idents: #field_types),*,
            paused: bool,
        }
    };
    let pausable_impl = quote! {
        impl Pausable for #ident {
            fn pause(&mut self) {
                self.paused = true;
            }
            fn unpause(&mut self) {
                self.paused = false;
            }
            fn is_paused(&self) -> bool {
                self.paused
            }
        }
    };

    Ok(quote! {
        #modified_struct
        #pausable_impl
    })
}
