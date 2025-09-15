use proc_macro2::{Ident, TokenStream};
use proc_macro_error2::abort_call_site;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, TypeArray};

use crate::shared::{self, fallback::Fallback, unreachable, BitSize};

pub(crate) fn default_bits(item: TokenStream) -> TokenStream {
    let derive_input = parse(item);
    //TODO: does fallback need handling?
    let (derive_data, name, ..) = analyze(&derive_input);

    match derive_data {
        Data::Struct(data) => generate_struct_default_impl(name, &data.fields),
        Data::Enum(_) => abort_call_site!("use derive(Default) for enums"),
        _ => unreachable(()),
    }
}

fn default(ty: &Type) -> TokenStream {
    quote! { <#ty as ::core::default::Default>::default() }
}

fn generate_default_array(array: &TypeArray) -> TokenStream {
    let inner = match array.elem.as_ref() {
        Type::Array(nested) => generate_default_array(&nested),
        v => default(&v),
    };

    let len = &array.len;
    quote! { [#inner; #len] }
}

fn generate_struct_default_impl(name: &Ident, fields: &Fields) -> TokenStream {
    let to_copy = match fields {
        Fields::Named(fields) => fields.named.iter(),
        Fields::Unnamed(fields) => fields.unnamed.iter(),
        Fields::Unit => unreachable!(),
    };

    let copies = to_copy.filter_map(|f| {
        if f.ident.as_ref().map(|i| i.to_string().starts_with("reserved_")).unwrap_or(false) {
            None
        } else {
            if let Type::Array(array) = &f.ty {
                Some(generate_default_array(&array))
            } else {
                Some(default(&f.ty))
            }
        }
    });

    quote! {
        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self::new(#(#copies,)*)
            }
        }
    }
}

fn parse(item: TokenStream) -> DeriveInput {
    shared::parse_derive(item)
}

fn analyze(derive_input: &DeriveInput) -> (&Data, &Ident, BitSize, Option<Fallback>) {
    shared::analyze_derive(derive_input, false)
}
