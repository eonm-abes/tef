extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::{parse, parse_macro_input, ItemStruct};

/// Add attribute #[serde(default)] for each fields of a struct.
#[proc_macro_attribute]
pub fn lax(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in &mut fields.named {
            let attr: Attribute = syn::parse_quote! {
                #[serde(default)]
            };
            field.attrs.push(attr);
        }
    }

    return quote!(
        #item_struct
    ).into();
}