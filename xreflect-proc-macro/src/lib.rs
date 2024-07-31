mod reflect_enum;
mod reflect_struct;
mod wrapper_types;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::{reflect_enum::reflect_enum, reflect_struct::reflect_struct, wrapper_types::EnumData};
use crate::wrapper_types::StructData;

#[proc_macro_derive(Reflect)]
pub fn reflect(input: TokenStream) -> TokenStream {
	let DeriveInput {
		ident,
		generics,
		data,
		..
	} = parse_macro_input!(input as DeriveInput);
	match data {
		Data::Struct(data_struct) => {
			reflect_struct(StructData::new(ident.to_string(), generics, data_struct)).into()
		}
		Data::Enum(data_enum) => reflect_enum(EnumData::new(
			ident.to_string(),
			generics,
			data_enum,
		))
		.into(),
		Data::Union(_) => unimplemented!(),
	}
}
