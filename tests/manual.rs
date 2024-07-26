#![feature(specialization)]

use xreflect::{EnumReflectInternal, StructLikeData};

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
		StructLikeData::Record(&[("field", Some("::core::primitive::u8"))])
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

	fn create_member_from_raw_parts(member_name: &str, member_data: StructLikeData) {
		todo!()
	}

	fn get_field_from_index<T>(&self, index: usize) -> Result<&T, ()> {
		use std::mem;

		match self {
			Self::Unit => {
				return Err(());
			}
			Self::Struct { field } => {
				match index {
					0 => {
						if !T::is_type_of(field) {
							return Err(());
						}
						unsafe { mem::transmute(field) }
					},
					_ => Err(())
				}
			}
			Self::Tuple(a) => {
				match index {
					0 => {
						if !T::is_type_of(a) {
							return Err(());
						}
						unsafe { mem::transmute(a) }
					},
					_ => Err(())
				}
			}
		}
	}

	fn get_field_from_index_mut<T>(&mut self, index: usize) -> Result<&mut T, ()> {
		todo!()
	}
}
trait IsType<T> {
	fn is_type_of(x: &T) -> bool;
}
impl<T> IsType<T> for T {
	default fn is_type_of(x: &T) -> bool {
		false
	}
}
#[test]
fn main() {
	let a = Test::Tuple(32);
	let b = a.get_field_from_index::<i32>(1);
	dbg!(b);
}