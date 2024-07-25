use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataStruct, Generics, Ident, Visibility};

pub(crate) fn reflect_struct(
	attrs: Vec<Attribute>,
	vis: Visibility,
	ident: Ident,
	generics: Generics,
	data_struct: DataStruct,
) -> TokenStream {
	let DataStruct { fields, .. } = data_struct;
	quote! {

	}
}
