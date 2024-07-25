extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

#[proc_macro_derive(Reflect)]
pub fn reflect(input: TokenStream) -> TokenStream {
	let DeriveInput {
		attrs, vis, ident, generics, data
	} = parse_macro_input!(input as DeriveInput);
	match data {
		Data::Struct(data_struct) => {

		}
		Data::Enum(data_enum) => {

		}
		Data::Union(_) => unimplemented!()
	}
	todo!()
}

pub trait Reflect {}