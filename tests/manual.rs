#![feature(specialization)]

use xreflect::{EnumReflectInternal, StructLikeData};
use xreflect_core::Builder;
use std::any::Any;
pub enum Test {
	Unit,
	Tuple(i32),
	Struct {
		field: u8
	}
}

impl EnumReflectInternal for Test {
	const MEMBERS: &'static [StructLikeData] = &[
		StructLikeData::Unit,
		StructLikeData::Tuple(&[Some("::core::primitive::i32")]),
		StructLikeData::Record(&[Some("::core::primitive::u8")])
	];
	const MEMBER_NAMES: &'static [&'static str] = &[];
	const TYPE_PATH: &'static str = "crate::Test";
	const TYPE_NAME: &'static str = "Test";

	fn get_index_of_member_name(member_name: &str) -> Result<usize, ()> {
		match member_name {
			"Unit" => Ok(0),
			"Tuple" => Ok(1),
			"Struct" => Ok(2),
			_ => Err(())
		}
	}
	fn get_field_from_index<T: 'static>(&self, index: usize) -> Result<&T, ()> {
		match self {
			Self::Unit => {
				return Err(());
			}
			Self::Struct { field } => {
				match index {
					0 => {
						(field as &dyn Any).downcast_ref::<T>().ok_or(())
					},
					_ => Err(())
				}
			}
			Self::Tuple(field) => {
				match index {
					0 => (field as &dyn Any).downcast_ref::<T>().ok_or(()),
					_ => Err(())
				}
			}
		}
	}

	fn get_field_from_index_mut<T>(&mut self, index: usize) -> Result<&mut T, ()> {
		match self {
			Self::Unit => {
				return Err(());
			}
			Self::Struct { field } => {
				match index {
					0 => {
						(field as &mut dyn Any).downcast_mut::<T>().ok_or(())
					},
					_ => Err(())
				}
			}
			Self::Tuple(field) => {
				match index {
					0 => (field as &mut dyn Any).downcast_mut::<T>().ok_or(()),
					_ => Err(())
				}
			}
		}
	}
}
pub(super) mod xreflect_builders0x121839438923924398234898924 {
	#![allow(non_snake_case)]
	pub mod enum_Test {
		struct Unit;
		struct Tuple(i32);
		struct Struct { field: u8 }
	}
}
#[test]
fn main() {
	let a = Test::Tuple(36);
	let b = a.get_field_from_index::<i32>(0);
	dbg!(b);
}