#![feature(specialization)]

use xreflect::{EnumReflectInternal, StructLikeData};
use xreflect_core::{Builder, EnumBuildable};
use std::any::Any;
enum Test {
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
impl<B: Builder<Test>> EnumBuildable<B> for Test {
	fn build(enum_member_name: &'static str) -> B {
		match enum_member_name {
			"Unit" => {
				// i seriously doubt that i need transmute, but whatever makes the type checker happy ig
				return unsafe { std::mem::transmute(__xreflect_builders0x121839438923924398234898924::enum_Test::Unit::new()) } ;
			},
			"Tuple" => {
				return unsafe { std::mem::transmute(__xreflect_builders0x121839438923924398234898924::enum_Test::Tuple::new()) } ;
			},
			"Struct" => {
				return unsafe { std::mem::transmute(__xreflect_builders0x121839438923924398234898924::enum_Test::Struct::new()) } ;
			},
			_ => panic!()
		}
	}
}
mod __xreflect_builders0x121839438923924398234898924 {
	#![allow(non_snake_case)]
	// make visibility reflect actual struct/enum
	pub(crate) mod enum_Test {
		use ::xreflect_core::Builder;
		use super::super::Test;

		#[derive(Default)]
		pub(crate) struct Unit;
		impl Builder<Test> for Unit {
			fn new() -> Self {
				Self::default()
			}
			fn with_field<T: 'static>(&mut self, _field_name: &'static str, _field_value: T) {}

			fn with_field_at<T: 'static>(&mut self, _field_index: usize, _field_value: T) {}

			fn try_build(self) -> Result<Test, ()> {
				Ok(Self::Unit)
			}
		}
		#[derive(Default)]
		pub struct Tuple(pub Option<i32>);
		impl Builder<Test> for Tuple {
			fn new() -> Self {
				Self::default()
			}
			fn with_field<T: 'static>(&mut self, field_name: &'static str, field_value: T) {
				match field_name {
					"0" => {
						if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
							self.0 = Some(unsafe { std::mem::transmute::<T, i32>(field_value) });
						} else {
							panic!()
						}
					},
					_ => panic!()
				}
			}
			fn with_field_at<T: 'static>(&mut self, field_index: usize, field_value: T) {
				match field_index {
					0 => {
						if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
							self.0 = Some(unsafe { std::mem::transmute::<T, i32>(field_value) });
						} else {
							panic!()
						}
					},
					_ => panic!()
				}
			}

			fn try_build(self) -> Result<Test, ()> {
				let Some(field0) = self.0 else { return Err(()) };
				Ok(Self::Tuple(field0))
			}
		}

		#[derive(Default)]
		pub struct Struct { pub(crate) field: Option<u8> }
		impl Builder<Test> for Struct {
			fn new() -> Self {
				Self::default()
			}
			fn with_field<T: 'static>(&mut self, field_name: &'static str, field_value: T) {
				match field_name {
					"field" => {
						if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
							self.field = Some(unsafe { std::mem::transmute::<T, u8>(field_value) });
						} else {
							panic!()
						}
					},
					_ => panic!()
				}
			}
			fn with_field_at<T: 'static>(&mut self, field_index: usize, field_value: T) {
				match field_index {
					0 => {
						if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<T>() {
							self.field = Some(unsafe { std::mem::transmute::<T, u8>(field_value) });
						} else {
							panic!()
						}
					},
					_ => panic!()
				}
			}

			fn try_build(self) -> Result<Test, ()> {
				let Some(field0) = self.field else { return Err(()) };
				Ok(Self::Field(field0))
			}
		}
	}
}
#[test]
fn main() {
	let a = Test::Tuple(36);
	let b = a.get_field_from_index::<i32>(0);
	dbg!(b);
	Test::build("Struct")
		.with_field("field", 100)
		.build();
}