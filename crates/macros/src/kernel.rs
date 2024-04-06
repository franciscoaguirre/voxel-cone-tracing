use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Ident, Result};

pub(crate) fn derive_kernel_inner(input: DeriveInput) -> Result<TokenStream2> {
    let kernel_impl = match &input.data {
        Data::Enum(data_enum) => kernel_enum_impl(&input.ident, data_enum),
        Data::Struct(data_struct) => kernel_struct_impl(&input.ident, data_struct),
        _ => return Err(Error::new_spanned(&input, "Expected a struct or an enum")),
    };

    Ok(quote! {
        #kernel_impl
    })
}

fn kernel_enum_impl(ident: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    let kernel_impl = quote! {
        impl Kernel for #ident {
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
    kernel_impl
}

fn kernel_struct_impl(ident: &Ident, data_struct: &DataStruct) -> TokenStream2 {
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect();
    let field_types: Vec<_> = data_struct.fields.iter().map(|field| &field.ty).collect();
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
    kernel_impl
}

pub(crate) fn derive_pausable_inner(input: DeriveInput) -> Result<TokenStream2> {
    let pausable_impl = match &input.data {
        Data::Enum(data_enum) => pausable_enum_impl(&input.ident, data_enum),
        Data::Struct(data_struct) => pausable_struct_impl(&input.ident, data_struct),
        _ => return Err(Error::new_spanned(&input, "Expected struct or enum.")),
    };

    Ok(quote! {
        #pausable_impl
    })
}

fn pausable_enum_impl(ident: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    let pausable_impl = quote! {
        impl Pausable for #ident {
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
    pausable_impl
}

// TODO: Should return a custom error if struct doesn't have the `paused` field.
fn pausable_struct_impl(ident: &Ident, _data_struct: &DataStruct) -> TokenStream2 {
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
    pausable_impl
}
