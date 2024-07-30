use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataStruct, Generics, Ident, Visibility};
use crate::wrapper_types::StructData;

pub(crate) fn reflect_struct(
	struct_data: StructData
) -> TokenStream {

	quote! {}
}
