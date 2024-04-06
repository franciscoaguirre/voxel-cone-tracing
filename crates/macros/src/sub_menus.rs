use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, Result};

pub(crate) fn simplify_sub_menus_inner(input: DeriveInput) -> Result<TokenStream2> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => generate_for_fields(input.ident, fields),
        _ => return Err(Error::new_spanned(&input, "Expected a named struct")),
    }
}

fn generate_for_fields(name: Ident, fields: &FieldsNamed) -> Result<TokenStream2> {
    let field_idents: Vec<_> = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = fields.named.iter().map(|field| field.ty.clone()).collect();
    let sub_menus_impl = quote! {
        impl #name {
            pub fn any_showing(&self) -> bool {
                #( self.#field_idents.is_showing() )||*
            }
        }
    };
    let sub_menu_inputs = quote! {
        type SubMenuInputs<'a> = (
            #( <#field_types as SubMenu>::InputData<'a>, )*
        );
    };
    let sub_menu_outputs = quote! {
        type SubMenuOutputs<'a> = (
            #( &'a <#field_types as SubMenu>::OutputData, )*
        );
    };
    // We need to iterate again because we need the indices
    let render_sub_menus = fields.named.iter().enumerate().map(|(index, field)| {
        let ident = &field.ident;
        quote! { self.sub_menus.#ident.render(ui.context(), &inputs.#index); }
    });
    let sub_menu_names = fields.named.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        ident
            .to_string()
            .split("_")
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    });
    let menu_impl = quote! {
        impl Menu {
            pub fn render(&mut self, inputs: SubMenuInputs) {
                let ui = Ui::instance();
                #( #render_sub_menus )*
            }

            pub fn get_data(&self) -> SubMenuOutputs {
                (
                    #( self.sub_menus.#field_idents.get_data(), )*
                )
            }

            pub fn show_main_window(&mut self) {
                let ui = Ui::instance();
                egui::Window::new("Menu").show(ui.context(), |ui| {
                    #(
                        if ui.button(get_button_text(
                            #sub_menu_names,
                            self.sub_menus.#field_idents.is_showing()
                        )).clicked() {
                            self.sub_menus.#field_idents.toggle_showing();
                        }
                    )*
                });
            }
        }
    };
    Ok(quote! {
        #sub_menus_impl
        #sub_menu_inputs
        #sub_menu_outputs
        #menu_impl
    })
}

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
        impl SubMenu for #ident {
            fn show(&self, context: &engine::ui::egui::Context, scene: &Scene, assets: &mut AssetRegistry) {
                match self {
                    #(Self::#variant_idents(inner_submenu) => {
                        if !inner_submenu.should_show() {
                            inner_submenu.show(context, scene, assets);
                        }
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
