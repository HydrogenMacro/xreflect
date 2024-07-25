use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DataEnum, Generics, Ident, Visibility};
use xreflect_core::{EnumReflect, StructType};
pub(crate) fn reflect_enum(
	attrs: Vec<Attribute>,
	vis: Visibility,
	ident: Ident,
	generics: Generics,
	data_enum: DataEnum,
) -> TokenStream {
	quote! {
		impl ::xreflect::EnumReflect for #ident {
			const STRUCT_TYPE: ::xreflect::StructType;
			fn type_name() -> &'static str {
				todo!()
			}
			fn get_field<T>(&self, _field_name: &str) -> &T {
				todo!()
			}
			fn set_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Result<(), ReflectError> {
				todo!()
			}
			fn with_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Self {
				todo!()
			}
			fn get_element<T>(&self, _element_index: usize) -> &T {
				todo!()
			}
			fn set_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Result<(), ReflectError> {
				todo!()
			}
			fn with_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Self {
				todo!()
			}
		}
	}
}