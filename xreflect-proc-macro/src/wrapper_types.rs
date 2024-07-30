use std::{collections::HashMap, hash::Hash, iter};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, DataStruct, Fields, GenericParam, Generics, Token, Type, WherePredicate};
pub(crate) struct EnumData {
	pub name: String,
	pub generics: Vec<GenericType>,
	pub variants: Vec<EnumVariantData>,
}
impl EnumData {
	pub fn new(enum_name: String, syn_generics: Generics, syn_enum: DataEnum) -> Self {
		let mut generics_store: HashMap<String, GenericType> = HashMap::new();
		let mut generics_order: Vec<String> = vec![];
		for generic in syn_generics.params {
			match generic {
				GenericParam::Const(const_generic) => {
					generics_store.insert(
						const_generic.ident.to_string(),
						GenericType::Const {
							name: const_generic.ident.to_string(),
							const_type: const_generic.ty,
						},
					);
					generics_order.push(const_generic.ident.to_string());
				}
				GenericParam::Lifetime(lifetime_generic) => {
					let lifetime_generic_bounds = if lifetime_generic.bounds.is_empty() {
						Some(lifetime_generic.bounds.to_token_stream())
					} else {
						None
					};
					generics_store.insert(
						lifetime_generic.lifetime.to_string(),
						GenericType::Lifetime {
							name: lifetime_generic.lifetime.to_string(),
							bounds: lifetime_generic_bounds,
						},
					);
					generics_order.push(lifetime_generic.lifetime.to_string());
				}
				GenericParam::Type(generic_type) => {
					let generic_type_bounds = if generic_type.bounds.is_empty() {
						Some(generic_type.bounds.to_token_stream())
					} else {
						None
					};
					generics_store.insert(
						generic_type.ident.to_string(),
						GenericType::Lifetime {
							name: generic_type.ident.to_string(),
							bounds: generic_type_bounds,
						},
					);
					generics_order.push(generic_type.ident.to_string());
				}
			}
		}
		if let Some(where_clause) = syn_generics.where_clause {
			for bound_stmt in where_clause.predicates {
				match bound_stmt {
					WherePredicate::Lifetime(lifetime_bound_stmt) => {
						let GenericType::Lifetime { ref mut bounds, .. } =
							generics_store.get_mut(&lifetime_bound_stmt.lifetime.ident.to_string()).unwrap() else { unreachable!() };
						*bounds = Some(lifetime_bound_stmt.bounds.to_token_stream());
					}
					WherePredicate::Type(type_bound_stmt) => {
						let GenericType::Type { ref mut bounds, .. } = generics_store
							.get_mut(&type_bound_stmt.bounded_ty.to_token_stream().to_string()).unwrap() else { unreachable!() };
						*bounds = Some(type_bound_stmt.bounds.to_token_stream());
					}
					_ => unimplemented!(),
				}
			}
		}
		let mut variants = vec![];
		for syn_variant in syn_enum.variants {
			let variant_name = syn_variant.ident.to_string();
			let variant_type = match syn_variant.fields {
				Fields::Named(record_fields) => StructType::Struct(
					record_fields
						.named
						.into_iter()
						.map(|record_field| {
							RecordEntry(record_field.ident.unwrap().to_string(), record_field.ty)
						})
						.collect(),
				),
				Fields::Unnamed(tuple_fields) => StructType::Tuple(
					tuple_fields
						.unnamed
						.into_iter()
						.map(|tuple_fields| TupleEntry(tuple_fields.ty))
						.collect(),
				),
				Fields::Unit => StructType::Unit,
			};
			variants.push(EnumVariantData {
				name: variant_name,
				variant_type,
			})
		}
		EnumData {
			name: enum_name,
			generics: generics_order
				.iter()
				.map(|generic_name| generics_store.remove(generic_name).unwrap())
				.collect(),
			variants,
		}
	}
}
pub(crate) struct EnumVariantData {
	pub name: String,
	pub variant_type: StructType,
}
pub(crate) enum StructType {
	Unit,
	Tuple(Vec<TupleEntry>),
	Struct(Vec<RecordEntry>),
}
impl StructType {
	const UNIT_DISCRIMINANT: usize = 0;
	const TUPLE_DISCRIMINANT: usize = 1;
	const STRUCT_DISCRIMINANT: usize = 2;
	pub fn amount_of_fields(&self) -> usize {
		match self {
			StructType::Unit => 0,
			StructType::Tuple(tuple_entries) => tuple_entries.len(),
			StructType::Struct(struct_entries) => struct_entries.len()
		}
	}
	pub fn match_ending_syntax(&self) -> TokenStream {
		match self {
			StructType::Unit => quote!{},
			StructType::Tuple(tuple_entries) => {
				let a = iter::repeat(quote!(_)).take(tuple_entries.len());
				quote! {
					(#(#a),*)
				}
			}
			StructType::Struct(record_entries) => {
				quote! {
					{ .. }
				}
			}
		}
	}
	pub fn match_ending_syntax_full(&self) -> TokenStream {
		match self {
			StructType::Unit => quote!{},
			StructType::Tuple(tuple_entries) => {
				let tuple_entries = (0..tuple_entries.len()).map(|i|format_ident!("field{}", i));
				quote! {
					(#(#tuple_entries),*)
				}
			}
			StructType::Struct(record_entries) => {
				let record_entry_names = record_entries.iter()
					.map(|record_entry| record_entry.0.clone());
				quote! {
					{ #(#record_entry_names),* }
				}
			}
		}
	}
	pub fn field_names(&self) -> Vec<String> {
		match self {
			Self::Unit => Vec::new(),
			Self::Tuple(tuple_entries) => (0usize..tuple_entries.len()).map(|i| i.to_string()).collect(),
			Self::Struct(record_entries) => record_entries.iter().map(|record_entry| record_entry.0.clone()).collect()
		}
	}
	pub fn discriminant(&self) -> usize {
		match self {
			Self::Unit => 0,
			Self::Tuple(_) => 1,
			Self::Struct(_) => 2
		}
	}
}
pub(crate) struct StructData {
	pub name: String,
	pub generics: Vec<GenericType>,
	pub struct_type: StructType,
}
impl StructData {
	pub fn new(enum_name: String, syn_generics: Generics, syn_struct: DataStruct) -> Self {
		let mut generics_store: HashMap<String, GenericType> = HashMap::new();
		let mut generics_order: Vec<String> = vec![];
		for generic in syn_generics.params {
			match generic {
				GenericParam::Const(const_generic) => {
					generics_store.insert(
						const_generic.ident.to_string(),
						GenericType::Const {
							name: const_generic.ident.to_string(),
							const_type: const_generic.ty,
						},
					);
					generics_order.push(const_generic.ident.to_string());
				}
				GenericParam::Lifetime(lifetime_generic) => {
					let lifetime_generic_bounds = if lifetime_generic.bounds.is_empty() {
						Some(lifetime_generic.bounds.to_token_stream())
					} else {
						None
					};
					generics_store.insert(
						lifetime_generic.lifetime.to_string(),
						GenericType::Lifetime {
							name: lifetime_generic.lifetime.to_string(),
							bounds: lifetime_generic_bounds,
						},
					);
					generics_order.push(lifetime_generic.lifetime.to_string());
				}
				GenericParam::Type(generic_type) => {
					let generic_type_bounds = if generic_type.bounds.is_empty() {
						Some(generic_type.bounds.to_token_stream())
					} else {
						None
					};
					generics_store.insert(
						generic_type.ident.to_string(),
						GenericType::Lifetime {
							name: generic_type.ident.to_string(),
							bounds: generic_type_bounds,
						},
					);
					generics_order.push(generic_type.ident.to_string());
				}
			}
		}
		if let Some(where_clause) = syn_generics.where_clause {
			for bound_stmt in where_clause.predicates {
				match bound_stmt {
					WherePredicate::Lifetime(lifetime_bound_stmt) => {
						let GenericType::Lifetime { ref mut bounds, .. } =
							generics_store.get_mut(&lifetime_bound_stmt.lifetime.ident.to_string()).unwrap() else { unreachable!() };
						*bounds = Some(lifetime_bound_stmt.bounds.to_token_stream());
					}
					WherePredicate::Type(type_bound_stmt) => {
						let GenericType::Type { ref mut bounds, .. } = generics_store
							.get_mut(&type_bound_stmt.bounded_ty.to_token_stream().to_string()).unwrap() else { unreachable!() };
						*bounds = Some(type_bound_stmt.bounds.to_token_stream());
					}
					_ => unimplemented!(),
				}
			}
		}

		let struct_type = match syn_struct.fields {
			Fields::Named(record_fields) => StructType::Struct(
				record_fields
					.named
					.into_iter()
					.map(|record_field| {
						RecordEntry(record_field.ident.unwrap().to_string(), record_field.ty)
					})
					.collect(),
			),
			Fields::Unnamed(tuple_fields) => StructType::Tuple(
				tuple_fields
					.unnamed
					.into_iter()
					.map(|tuple_field| TupleEntry(tuple_field.ty))
					.collect(),
			),
			Fields::Unit => StructType::Unit,
		};
		StructData {
			name: enum_name,
			generics: generics_order
				.iter()
				.map(|generic_name| generics_store.remove(generic_name).unwrap())
				.collect(),
			struct_type,
		}
	}
}
pub(crate) struct RecordEntry(pub String, pub syn::Type);
pub(crate) struct TupleEntry(pub syn::Type);
pub(crate) enum GenericType {
	Lifetime {
		name: String,
		bounds: Option<TokenStream>,
	},
	Type {
		name: String,
		bounds: Option<TokenStream>,
	},
	Const {
		name: String,
		const_type: Type,
	},
}
