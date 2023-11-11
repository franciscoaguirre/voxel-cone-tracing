use proc_macro2::TokenStream as TokenStream2;
use syn::{
    DeriveInput, Result, Error, Ident, Fields, FieldsNamed, Data, DataStruct,
};
use quote::quote;

pub(crate) fn simplify_sub_menus_inner(input: DeriveInput) -> Result<TokenStream2> {
    match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => generate_for_fields(input.ident, fields),
        _ => return Err(Error::new_spanned(&input, "Expected a named struct")),
    }
}

fn generate_for_fields(name: Ident, fields: &FieldsNamed) -> Result<TokenStream2> {
    let field_idents: Vec<_> = fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect();
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
        ident.to_string()
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
