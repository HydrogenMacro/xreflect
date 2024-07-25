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
	let DataEnum { variants, .. } = data_enum;
	let enum_entries = variants.iter().map(|variant| variant.ident.clone()).collect::<Vec<Ident>>();
	quote! {
		impl ::xreflect::EnumReflect for #ident {
			const MEMBERS: ::xreflect::phf::Map<&'static str, StructType> = ::xreflect::phf::phf_map! {

			};
			fn type_name_of_member(_member: &str) -> &'static str {
				todo!()
			}
			fn construct(_enum_member: &str) -> Self {
				todo!()
			}
			fn get_field<T>(&self, _field_name: &str) -> &T {
				todo!()
			}
			fn set_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Result<(), ReflectError> {
				todo!()
			}
			fn has_field(&self, _field_name: &str) -> bool { todo!() }
			fn with_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Self {
				todo!()
			}
			fn get_element<T>(&self, _element_index: usize) -> &T {
				todo!()
			}
			fn set_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Result<(), ReflectError> {
				todo!()
			}
			fn has_element(&self, _element_index: usize) -> bool { todo!() }
			fn with_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Self {
				todo!()
			}
		}
	}
}