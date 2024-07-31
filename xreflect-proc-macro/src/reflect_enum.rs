use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

use crate::wrapper_types::{EnumData, StructType};

pub(crate) fn reflect_enum(enum_data: EnumData) -> TokenStream {
	let enum_name_ident = format_ident!("{}", enum_data.name);

	// amount_of_fields() method
	let mut amount_of_fields_branches = vec![];
	for (_, enum_variant_data) in enum_data.variants.iter().enumerate() {
		let enum_variant_name_ident = Ident::new(&enum_variant_data.name, Span::call_site());
		let match_ending_syntax = enum_variant_data.variant_type.match_ending_syntax();
		let amount_of_fields = enum_variant_data.variant_type.amount_of_fields();
		amount_of_fields_branches.push(
			quote!{
				Self::#enum_variant_name_ident #match_ending_syntax => #amount_of_fields,
			}
		)
	}

	// try_get_index_of_field() method
	let mut try_get_index_of_field_branches = vec![];
	for (_, enum_variant_data) in enum_data.variants.iter().enumerate() {
		let enum_variant_name_ident = Ident::new(&enum_variant_data.name, Span::call_site());
		let match_ending_syntax = enum_variant_data.variant_type.match_ending_syntax();
		let member_name_branches = match &enum_variant_data.variant_type {
			StructType::Unit => vec![],
			StructType::Tuple(tuple_entries) => {
				tuple_entries.iter().enumerate().map(|(i, _)| {
					let i_as_string = i.to_string();
					quote! {
						#i_as_string => Ok(#i),
					}
				}).collect()
			}
			StructType::Struct(record_entries) => {
				record_entries.iter().enumerate().map(|(i, record_entry)| {
					let record_name = record_entry.0.clone();
					quote! {
						#record_name => Ok(#i),
					}
				}).collect()
			}
		};
		try_get_index_of_field_branches.push(
			quote!{
				Self::#enum_variant_name_ident #match_ending_syntax => {
					match member_name {
						#(#member_name_branches)*
						_ => Err(::xreflect::ReflectError::FieldNotFound)
					}
				},
			}
		);
	}


	// try_get_field_at()/try_get_field_mut_at() methods
	let mut try_get_field_at_branches = vec![];
	let mut try_get_field_mut_at_branches = vec![];
	for (_, enum_variant_data) in enum_data.variants.iter().enumerate() {
		let enum_variant_name_ident = Ident::new(&enum_variant_data.name, Span::call_site());
		let match_ending_syntax_full = enum_variant_data.variant_type.match_ending_syntax_named();
		let mut field_match_index_branches = vec![];
		let mut field_mut_match_index_branches = vec![];
		match &enum_variant_data.variant_type {
			StructType::Unit => {},
			StructType::Tuple(tuple_entries) => {
				for i in 0..tuple_entries.len() {
					let field_name = format_ident!("field{}", i);
					field_match_index_branches.push(
						quote! {
							#i => (#field_name as &dyn ::core::any::Any)
								.downcast_ref::<T>()
								.ok_or(::xreflect::ReflectError::WrongType),
						}
					);
					field_mut_match_index_branches.push(
						quote! {
							#i => (#field_name as &mut dyn ::core::any::Any)
								.downcast_mut::<T>()
								.ok_or(::xreflect::ReflectError::WrongType),
						}
					);
				}
			},
			StructType::Struct(record_entries) => {
				for (i, record_entry) in record_entries.iter().enumerate() {
					let field_name = Ident::new(&record_entry.0, Span::call_site());
					field_match_index_branches.push(
						quote! {
							#i => (#field_name as &dyn ::core::any::Any)
								.downcast_ref::<T>()
								.ok_or(::xreflect::ReflectError::WrongType),
						}
					);
					field_mut_match_index_branches.push(
						quote! {
							#i => (#field_name as &mut dyn ::core::any::Any)
								.downcast_mut::<T>()
								.ok_or(::xreflect::ReflectError::WrongType),
						}
					);
				}
			}
		}
		try_get_field_at_branches.push(
			quote!{
				Self::#enum_variant_name_ident #match_ending_syntax_full => {
					match index {
						#(#field_match_index_branches)*
						_ => Err(::xreflect::ReflectError::FieldNotFound)
					}
				},
			}
		);
		try_get_field_mut_at_branches.push(
			quote!{
				Self::#enum_variant_name_ident #match_ending_syntax_full => {
					match index {
						#(#field_mut_match_index_branches)*
						_ => Err(::xreflect::ReflectError::FieldNotFound)
					}
				},
			}
		);
	}

	// try_get_type_of_field_at() method
	let mut try_get_type_of_field_at_branches = vec![];
	for (_, enum_variant_data) in enum_data.variants.iter().enumerate() {
		let enum_variant_name_ident = Ident::new(&enum_variant_data.name, Span::call_site());
		let match_ending_syntax = enum_variant_data.variant_type.match_ending_syntax();
		let mut field_index_branches = vec![];
		match &enum_variant_data.variant_type {
			StructType::Unit => {}
			StructType::Tuple(tuple_entries) => {
				for (i, tuple_entry) in tuple_entries.iter().enumerate() {
					let ty = &tuple_entry.0;
					field_index_branches.push(
						quote! {
							#i => Ok(::core::any::TypeId::of::<#ty>()),
						}
					)
				}
			}
			StructType::Struct(record_entries) => {
				for (i, record_entry) in record_entries.iter().enumerate() {
					let ty = &record_entry.1;
					field_index_branches.push(
						quote! {
							#i => Ok(::core::any::TypeId::of::<#ty>()),
						}
					)
				}
			}
		}
		try_get_type_of_field_at_branches.push(
			quote!{
				Self::#enum_variant_name_ident #match_ending_syntax => {
					match field_index {
						#(#field_index_branches)*
						_ => Err(::xreflect::ReflectError::FieldNotFound)
					}
				},
			}
		)
	}

	let reflect_impl = quote! {
		impl ::xreflect::Reflect for #enum_name_ident {
			fn amount_of_fields(&self) -> usize {
				match self {
					#(
						#amount_of_fields_branches
					)*
				}
			}

			fn try_get_index_of_field(&self, member_name: &str) -> Result<usize, ::xreflect::ReflectError> {
				match self {
					#(#try_get_index_of_field_branches)*
				}
			}
			fn try_get_field_at<T: 'static>(&self, index: usize) -> Result<&T, ::xreflect::ReflectError> {
				match self {
					#(#try_get_field_at_branches)*
				}
			}

			fn try_get_field_mut_at<T: 'static>(&mut self, index: usize) -> Result<&mut T, ::xreflect::ReflectError> {
				match self {
					#(#try_get_field_mut_at_branches)*
				}
			}

			fn try_get_type_of_field_at(&self, field_index: usize) -> Result<::core::any::TypeId, ::xreflect::ReflectError> {
				match self {
					#(#try_get_type_of_field_at_branches)*
				}
			}
		}
	};
	// std::fs::write("aaa.rs", reflect_impl.to_string()).unwrap();
	reflect_impl
}
