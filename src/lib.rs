pub use xreflect_core::*;
pub use xreflect_proc_macro::*;
use xreflect_core::{StructLikeData};

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

	fn get_field_from_index<T>(&self, index: usize) -> &T {
		match self {
			Self::Unit => {
				panic!()
			}
			Self::Struct { field } => {
				match index {
					0 => todo!(),
					_ => panic!()
				}
			}
			Self::Tuple(a) => {
				match index {
					0 => todo!(),
					_ => panic!()
				}
			}
		}
	}

	fn get_field_from_index_mut<T>(&mut self, index: usize) -> &mut T {
		todo!()
	}
}

trait IsMe {
	fn is_me<T>(possibly_me: T) -> bool {
		false
	}
}
impl<A> IsMe for A {
	fn is_me<T>(possibly_me: T) -> bool { false }
}
impl <A> IsMe for A {
	fn is_me<A>(possibly_me: A) -> bool { true }
}