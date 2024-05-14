use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Ident, Result};

pub(crate) fn derive_submenu_inner(input: DeriveInput) -> Result<TokenStream2> {
    let data_enum = match &input.data {
        Data::Enum(data_enum) => data_enum,
        _ => return Err(Error::new_spanned(&input, "Expected an enum")),
    };
    let ident = &input.ident;
    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    let submenu_impl = quote! {
        impl<SystemType: System + Pausable> SubMenu<SystemType> for #ident {
            fn show(&mut self, context: &engine::ui::egui::Context, inputs: &mut SubMenuInputs<SystemType>) {
                match self {
                    #(Self::#variant_idents(inner_submenu) => {
                        if inner_submenu.should_show() {
                            inner_submenu.show(context, inputs);
                        }
                    }),*
                }
            }

            fn handle_event(&mut self, event: &engine::ui::glfw::WindowEvent, context: &engine::ui::egui::Context, inputs: &mut SubMenuInputs<SystemType>) {
                match self {
                    #(Self::#variant_idents(inner_submenu) => {
                        inner_submenu.handle_event(event, context, inputs);
                    }),*
                }
            }
        }
    };

    Ok(submenu_impl)
}

pub(crate) fn derive_showable_inner(input: DeriveInput) -> Result<TokenStream2> {
    let showable_impl = match &input.data {
        Data::Enum(data_enum) => showable_enum_impl(&input.ident, data_enum),
        Data::Struct(data_struct) => showable_struct_impl(&input.ident, data_struct),
        _ => return Err(Error::new_spanned(&input, "Expected struct or enum.")),
    };

    Ok(quote! {
        #showable_impl
    })
}

fn showable_enum_impl(ident: &Ident, data_enum: &DataEnum) -> TokenStream2 {
    let variant_idents: Vec<_> = data_enum
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect();
    let showable_impl = quote! {
        impl Showable for #ident {
            fn toggle_showing(&mut self) {
                match self {
                    #(Self::#variant_idents(inner_submenu) => inner_submenu.toggle_showing()),*
                }
            }
            fn should_show(&self) -> bool {
                match self {
                    #(Self::#variant_idents(inner_submenu) => inner_submenu.should_show()),*
                }
            }
        }
    };
    showable_impl
}

// TODO: Should return a custom error if the struct doesn't have the `should_show` field.
fn showable_struct_impl(ident: &Ident, _data_struct: &DataStruct) -> TokenStream2 {
    let showable_impl = quote! {
        impl Showable for #ident {
            fn toggle_showing(&mut self) {
                self.should_show = !self.should_show;
            }
            fn should_show(&self) -> bool {
                self.should_show
            }
        }
    };
    showable_impl
}
