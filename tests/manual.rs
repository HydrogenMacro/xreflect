#![allow(unused)]

use xreflect_core::{Builder, EnumBuildable, Reflect, ReflectError, StructType};
use std::any::{Any, TypeId};

enum Test {
	Unit,
	Tuple(i32),
	Struct {
		field: u8
	},
}

impl Reflect for Test {
	fn amount_of_fields(&self) -> usize {
		match self {
			Test::Unit => 0,
			Test::Tuple(_) => 1,
			Test::Struct { .. } => 2
		}
	}

	fn get_index_of_field(&self, member_name: &str) -> Result<usize, ReflectError> {
		match member_name {
			"Unit" => Ok(0),
			"Tuple" => Ok(1),
			"Struct" => Ok(2),
			_ => Err(ReflectError::EnumMemberNotFound)
		}
	}
	fn get_field_at_index<T: 'static>(&self, index: usize) -> Result<&T, ReflectError> {
		match self {
			Self::Unit => {
				return Err(ReflectError::FieldNotFound);
			}
			Self::Struct { field } => {
				match index {
					0 => {
						(field as &dyn Any).downcast_ref::<T>().ok_or(ReflectError::WrongType)
					}
					_ => Err(ReflectError::FieldNotFound)
				}
			}
			Self::Tuple(field) => {
				match index {
					0 => (field as &dyn Any).downcast_ref::<T>().ok_or(ReflectError::WrongType),
					_ => Err(ReflectError::FieldNotFound)
				}
			}
		}
	}

	fn get_field_at_index_mut<T: 'static>(&mut self, index: usize) -> Result<&mut T, ReflectError> {
		match self {
			Self::Unit => {
				return Err(ReflectError::FieldNotFound);
			}
			Self::Struct { field } => {
				match index {
					0 => {
						(field as &mut dyn Any).downcast_mut::<T>().ok_or(ReflectError::WrongType)
					}
					_ => Err(ReflectError::FieldNotFound)
				}
			}
			Self::Tuple(field) => {
				match index {
					0 => (field as &mut dyn Any).downcast_mut::<T>().ok_or(ReflectError::WrongType),
					_ => Err(ReflectError::FieldNotFound)
				}
			}
		}
	}

	fn get_type_of_field(&self, field_name: &str) -> TypeId {
		match self {
			Test::Unit => {
				panic!()
			}
			Test::Tuple(_) => {}
			Test::Struct { .. } => {

			}
		}
	}

	fn get_type_of_field_at(&self, field_name: &str) -> TypeId {
		todo!()
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
	let b = a.get_field_at_index::<i32>(0);
	dbg!(b);
	Test::build("Tuple")
		.with_field_at(0, 100i32)
		.build();
}