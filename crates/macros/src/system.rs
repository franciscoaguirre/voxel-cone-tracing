use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Ident, Result};

pub(crate) fn derive_system_inner(input: DeriveInput) -> Result<TokenStream2> {
    let system_impl = match &input.data {
        Data::Enum(data_enum) => system_enum_impl(&input.ident, data_enum),
        Data::Struct(data_struct) => system_struct_impl(&input.ident, data_struct),
        _ => return Err(Error::new_spanned(&input, "Expected a struct or an enum")),
    };

    Ok(quote! {
        #system_impl
    })
}

fn system_enum_impl(ident: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    let system_impl = quote! {
        impl System for #ident {
            unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
                match self {
                    #(Self::#variant_idents(inner_system) => {
                        if !inner_system.is_paused() {
                            inner_system.setup(assets);
                        }
                    }),*
                }
            }

            unsafe fn update(&mut self, inputs: SystemInputs) {
                match self {
                    #(Self::#variant_idents(inner_system) => {
                        if !inner_system.is_paused() {
                            inner_system.update(inputs);
                        }
                    }),*
                }
            }

            fn get_info(&self) -> SystemInfo {
                match self {
                    #(Self::#variant_idents(inner_system) => inner_system.get_info()),*
                }
            }
        }
    };
    system_impl
}

fn system_struct_impl(ident: &Ident, data_struct: &DataStruct) -> TokenStream2 {
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|field| &field.ident)
        .collect();
    let system_impl = quote! {
        impl System for #ident {
            unsafe fn setup(&mut self, assets: &mut AssetRegistry) {
                #(self.#field_idents.setup(assets));*;
            }
            unsafe fn update(&mut self, inputs: SystemInputs) {
                #(self.#field_idents.update(inputs));*;
            }
            fn get_info(&self) -> SystemInfo {
                let mut group_name = String::new();
                #(group_name.push_str(self.#field_idents.get_info().name));*;
                SystemInfo { name: group_name }
            }
        }
    };
    system_impl
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
                    #(Self::#variant_idents(inner_system) => inner_system.pause()),*
                }
            }
            fn unpause(&mut self) {
                match self {
                    #(Self::#variant_idents(inner_system) => inner_system.unpause()),*
                }
            }
            fn is_paused(&self) -> bool {
                match self {
                    #(Self::#variant_idents(inner_system) => inner_system.is_paused()),*
                }
            }
            fn is_paused_mut(&mut self) -> &mut bool {
                match self {
                    #(Self::#variant_idents(inner_system) => inner_system.is_paused_mut()),*
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
            fn is_paused_mut(&mut self) -> &mut bool {
                &mut self.paused
            }
        }
    };
    pausable_impl
}
