use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataEnum, Generics, Ident, Visibility};
pub(crate) fn reflect_enum(
	attrs: Vec<Attribute>,
	vis: Visibility,
	ident: Ident,
	generics: Generics,
	data_enum: DataEnum,
) -> TokenStream {
	let DataEnum { variants, .. } = data_enum;
	let enum_entries = variants.iter().map(|variant| variant.ident.clone()).collect::<Vec<Ident>>();
	quote! {

	}
}