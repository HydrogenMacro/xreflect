#![feature(specialization)]

use xreflect::{EnumReflectInternal, StructLikeData};
use xreflect_core::{Builder, EnumBuildable, TypePath};
use std::any::Any;

enum Test {
	Unit,
	Tuple(i32),
	Struct {
		field: u8
	},
}

impl EnumReflectInternal for Test {
	const MEMBERS: &'static [StructLikeData] = &[
		StructLikeData::Unit,
		StructLikeData::Tuple(&[Some(TypePath::new("::core::primitive::i32"))]),
		StructLikeData::Record(&[Some(TypePath::new("::core::primitive::u8"))])
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
					}
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

	fn get_field_from_index_mut<T: 'static>(&mut self, index: usize) -> Result<&mut T, ()> {
		match self {
			Self::Unit => {
				return Err(());
			}
			Self::Struct { field } => {
				match index {
					0 => {
						(field as &mut dyn Any).downcast_mut::<T>().ok_or(())
					}
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

impl EnumBuildable for Test {
	fn build(enum_member_name: &'static str) -> impl Builder<Self> {
		match enum_member_name {
			"Unit" => {
				return __xreflect_builders0x121839438923924398234898924::TestBuilder::Unit;
			}
			"Tuple" => {
				return __xreflect_builders0x121839438923924398234898924::TestBuilder::Tuple(None);
			}
			"Struct" => {
				return __xreflect_builders0x121839438923924398234898924::TestBuilder::Struct { field: None };
			}
			_ => panic!()
		}
	}
}

mod __xreflect_builders0x121839438923924398234898924 {
	// reflect actual visibility
	pub(crate) enum TestBuilder {
		Unit,
		Tuple(Option<i32>),
		Struct { field: Option<u8> },
	}

	use ::xreflect_core::Builder;
	use super::Test;

	impl Builder<Test> for TestBuilder {
		fn with_field<T: 'static>(mut self, field_name: &'static str, field_value: T) -> Self {
			match self {
				Self::Unit => {}
				Self::Tuple(ref mut field0) => {
					match field_name {
						"0" => {
							if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
								*field0 = Some(unsafe { std::mem::transmute_copy::<T, i32>(&field_value) });
							} else {
								panic!()
							}
						}
						_ => panic!()
					}
				}
				Self::Struct { ref mut field } => {
					match field_name {
						"field" => {
							if std::any::TypeId::of::<u8>() == std::any::TypeId::of::<T>() {
								*field = Some(unsafe { std::mem::transmute_copy::<T, u8>(&field_value) });
							} else {
								panic!()
							}
						}
						_ => panic!()
					}
				}
			}
			self
		}

		fn with_field_at<T: 'static>(mut self, field_index: usize, field_value: T) -> Self {
			match self {
				Self::Unit => {}
				Self::Tuple(ref mut field0) => {
					match field_index {
						0 => {
							if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
								*field0 = Some(unsafe { std::mem::transmute_copy::<T, i32>(&field_value) });
							} else {
								panic!()
							}
						}
						_ => panic!()
					}
				}
				Self::Struct { ref mut field } => {
					match field_index {
						0 => {
							if std::any::TypeId::of::<u8>() == std::any::TypeId::of::<T>() {
								*field = Some(unsafe { std::mem::transmute_copy::<T, u8>(&field_value) });
							} else {
								panic!()
							}
						}
						_ => panic!()
					}
				}
			}
			self
		}

		fn try_build(self) -> Result<Test, ()> {
			match self {
				TestBuilder::Unit => {
					Ok(Test::Unit)
				}
				TestBuilder::Tuple(field0) => {
					let Some(field0) = field0 else { panic!() };
					Ok(Test::Tuple(field0))
				}
				TestBuilder::Struct { field } => {
					let Some(field) = field else { panic!() };
					Ok(Test::Struct { field })
				}
			}
		}
	}
}

#[test]
fn main() {
	let a = Test::Tuple(36);
	let b = a.get_field_from_index::<i32>(0);
	dbg!(b);
	Test::build("Tuple")
		.with_field_at(0, 100i32)
		.build();
}