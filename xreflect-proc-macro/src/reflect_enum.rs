use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::Index;

use crate::wrapper_types::EnumData;

pub(crate) fn reflect_enum(enum_data: EnumData) -> TokenStream {
	let enum_name_ident = format_ident!("{}", enum_data.name);
	let enum_variant_names = enum_data.variants.iter().map(|variant| variant.name.clone());
	let enum_variant_name_idents = enum_variant_names.clone().map(|enum_variant_name| Ident::new(&enum_variant_name, Span::call_site()));
	let enum_variant_match_syntax = enum_data.variants.iter().map(|variant| variant.variant_type.match_ending_syntax());
	let enum_variant_indexes = (0..enum_variant_names.len()).map(Index::from);
	let enum_variant_field_counts = enum_data.variants.iter().map(|variant| variant.variant_type.amount_of_fields());
	quote! {
		impl ::xreflect::Reflect for #enum_name_ident {
			fn amount_of_fields(&self) -> usize {
				match self {
					#(
						#enum_name_ident::#enum_variant_name_idents #enum_variant_match_syntax => #enum_variant_field_counts
					),*
				}
			}

			fn try_get_index_of_field(&self, member_name: &str) -> Result<usize, ::xreflect::ReflectError> {
				todo!()
			}
			fn try_get_field_at<T: 'static>(&self, index: usize) -> Result<&T, ::xreflect::ReflectError> {
				todo!()
			}

			fn try_get_field_mut_at<T: 'static>(&mut self, index: usize) -> Result<&mut T, ::xreflect::ReflectError> {
				todo!()
			}

			fn try_get_type_of_field_at(&self, field_index: usize) -> Result<::core::any::TypeId, ::xreflect::ReflectError> {
				todo!()
			}
		}
	}
}
