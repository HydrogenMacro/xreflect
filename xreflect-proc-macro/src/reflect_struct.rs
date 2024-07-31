use proc_macro2::{Ident, TokenStream, Span};
use quote::{format_ident, quote};
use syn::Index;
use crate::wrapper_types::{StructData, StructType};

pub(crate) fn reflect_struct(
	struct_data: StructData
) -> TokenStream {
	let struct_name_ident = format_ident!("{}", struct_data.name);

	// amount_of_fields() method
	let amount_of_fields = struct_data.struct_type.amount_of_fields();

	// try_get_index_of_field() method
	let mut try_get_index_of_field_branches = vec![];
	match struct_data.struct_type {
		StructType::Unit => {}
		StructType::Tuple(ref tuple_entries) => {
			for i in 0..tuple_entries.len() {
				let field_name = format_ident!("{}", i);
				try_get_index_of_field_branches.push(
					quote!(
						#field_name => Ok(#i),
					)
				);
			}
		}
		StructType::Struct(ref struct_entries) => {
			for (i, record_entry) in struct_entries.iter().enumerate() {
				let field_name = record_entry.0.clone();
				try_get_index_of_field_branches.push(
					quote!(
						#field_name => Ok(#i),
					)
				);
			}
		}
	}

	// try_get_field_at()/try_get_field_mut_at() methods
	let mut try_get_field_at_branches = vec![];
	let mut try_get_field_mut_at_branches = vec![];
	match struct_data.struct_type {
		StructType::Unit => {}
		StructType::Tuple(ref tuple_entries) => {
			let match_ending_syntax_ref = struct_data.struct_type.match_ending_syntax_ref();
			let match_ending_syntax_mut_ref = struct_data.struct_type.match_ending_syntax_ref_mut();
			for i in 0..tuple_entries.len() {
				let field_index = Index::from(i);
				try_get_field_at_branches.push(
					quote!(
						#i => {
							let Self #match_ending_syntax_ref = self;
							(#field_index as &dyn ::core::any::Any)
								.downcast_ref::<T>()
								.ok_or(::xreflect::ReflectError::WrongType)
						},
					)
				);
				try_get_field_mut_at_branches.push(
					quote!(
						#i => {
							let Self #match_ending_syntax_mut_ref = self;
							(#field_index as &mut dyn ::core::any::Any)
								.downcast_mut::<T>()
								.ok_or(::xreflect::ReflectError::WrongType)
						},
					)
				);
			}
		}
		StructType::Struct(ref struct_entries) => {
			let match_ending_syntax_ref = struct_data.struct_type.match_ending_syntax_ref();
			let match_ending_syntax_mut_ref = struct_data.struct_type.match_ending_syntax_ref_mut();
			for (i, record_entry) in struct_entries.iter().enumerate() {
				let field_name = Ident::new(&record_entry.0, Span::call_site());
				try_get_field_at_branches.push(
					quote!(
						#i => {
							let Self #match_ending_syntax_ref = self;
							(#field_name as &dyn ::core::any::Any)
							.downcast_ref::<T>()
							.ok_or(::xreflect::ReflectError::WrongType)
						},
					)
				);
				try_get_field_mut_at_branches.push(
					quote!(
						#i => {
							let Self #match_ending_syntax_mut_ref = self;
							(#field_name as &mut dyn ::core::any::Any)
							.downcast_mut::<T>()
							.ok_or(::xreflect::ReflectError::WrongType)
						},
					)
				);
			}
		}
	}

	// try_get_type_of_field_at() method
	let mut try_get_type_of_field_at_branches = vec![];
	match struct_data.struct_type {
		StructType::Unit => {}
		StructType::Tuple(ref tuple_entries) => {
			for (i, tuple_entry) in tuple_entries.iter().enumerate() {
				let tuple_entry_type = &tuple_entry.0;
				try_get_type_of_field_at_branches.push(
					quote!(
						#i => Ok(::core::any::TypeId::of::<#tuple_entry_type>()),
					)
				);
			}
		}
		StructType::Struct(ref struct_entries) => {
			for (i, record_entry) in struct_entries.iter().enumerate() {
				let record_entry_type = &record_entry.1;
				try_get_type_of_field_at_branches.push(
					quote!(
						#i => Ok(::core::any::TypeId::of::<#record_entry_type>()),
					)
				);
			}
		}
	}
	let reflect_impl = quote! {
		impl ::xreflect::Reflect for #struct_name_ident {
			fn amount_of_fields(&self) -> usize {
				#amount_of_fields
			}

			fn try_get_index_of_field(&self, field_name: &str) -> Result<usize, ::xreflect::ReflectError> {
				match field_name {
					#(#try_get_index_of_field_branches)*
					_ => Err(::xreflect::ReflectError::FieldNotFound)
				}
			}

			fn try_get_field_at<T: 'static>(&self, field_index: usize) -> Result<&T, ::xreflect::ReflectError> {
				match field_index {
					#(#try_get_field_at_branches)*
					_ => Err(::xreflect::ReflectError::FieldNotFound)
				}
			}

			fn try_get_field_mut_at<T: 'static>(&mut self, field_index: usize) -> Result<&mut T, ::xreflect::ReflectError> {
				match field_index {
					#(#try_get_field_mut_at_branches)*
					_ => Err(::xreflect::ReflectError::FieldNotFound)
				}
			}

			fn try_get_type_of_field_at(&self, field_index: usize) -> Result<::core::any::TypeId, ::xreflect::ReflectError> {
				match field_index {
					#(#try_get_type_of_field_at_branches)*
					_ => Err(::xreflect::ReflectError::FieldNotFound)
				}
			}
		}
	};
	// for debugging
	// std::fs::write("bbb.rs", reflect_impl.to_string()).unwrap();
	reflect_impl
}
