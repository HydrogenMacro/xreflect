#![feature(const_type_id)]

use std::any::{Any, TypeId};

use xreflect::{Reflect, ReflectError, StructReflect, StructType};

struct Test<T: 'static> {
	a: i32,
	b: T,
}
impl<N: 'static> Reflect for Test<N> {
	fn amount_of_fields(&self) -> usize {
		2
	}

	fn try_get_index_of_field(&self, field_name: &str) -> Result<usize, ReflectError> {
		match field_name {
			"a" => Ok(0),
			"b" => Ok(1),
			_ => Err(ReflectError::FieldNotFound),
		}
	}

	fn try_get_field_at<T: 'static>(&self, field_index: usize) -> Result<&T, ReflectError> {
		match field_index {
			0 => (&self.a as &dyn Any)
				.downcast_ref::<T>()
				.ok_or(ReflectError::WrongType),
			1 => (&self.b as &dyn Any)
				.downcast_ref::<T>()
				.ok_or(ReflectError::WrongType),
			_ => Err(ReflectError::FieldNotFound),
		}
	}

	fn try_get_field_mut_at<T: 'static>(
		&mut self,
		field_index: usize,
	) -> Result<&mut T, ReflectError> {
		match field_index {
			0 => (&mut self.a as &mut dyn Any)
				.downcast_mut::<T>()
				.ok_or(ReflectError::WrongType),
			1 => (&mut self.b as &mut dyn Any)
				.downcast_mut::<T>()
				.ok_or(ReflectError::WrongType),
			_ => Err(ReflectError::FieldNotFound),
		}
	}

	fn try_get_type_of_field_at(&self, field_index: usize) -> Result<TypeId, ReflectError> {
		match field_index {
			0 => Ok(TypeId::of::<i32>()),
			1 => Ok(TypeId::of::<N>()),
			_ => Err(ReflectError::FieldNotFound),
		}
	}
}
impl<N: 'static> StructReflect for Test<N> {
	const TYPE: StructType =
		StructType::Record(&[("a", TypeId::of::<i32>()), ("b", TypeId::of::<N>())]);
	const FIELD_NAMES: &'static [&'static str] = &["a", "b"];

	fn try_get_index_of_field(field_name: &str) -> Result<usize, ReflectError> {
		match field_name {
			"aaa" => Ok(0),
			"b" => Ok(1),
			_ => Err(ReflectError::FieldNotFound),
		}
	}
}

mod __xreflect_builders0x121839438923924398234898924 {
	use xreflect_core::Builder;

	use crate::Test;

	pub(crate) struct TestBuilder<T> {
		a: Option<i32>,
		b: Option<T>,
	}
	impl<N> Builder<Test<N>> for TestBuilder<N> {
		fn with_field<T: 'static>(mut self, field_name: &'static str, field_value: T) -> Self {
			match field_name {
				"a" => {
					if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
						self.a = Some(unsafe { std::mem::transmute_copy::<T, i32>(&field_value) });
					} else {
						panic!("Wrong type")
					}
				}
				"b" => {
					if std::any::TypeId::of::<T>() == std::any::TypeId::of::<N>() {
						self.b = Some(unsafe { std::mem::transmute_copy::<T, N>(&field_value) });
					} else {
						panic!("Wrong type")
					}
				}
				_ => panic!("Field not found"),
			}
			self
		}

		fn with_field_at<T: 'static>(mut self, field_index: usize, field_value: T) -> Self {
			match field_index {
				0 => {
					if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
						self.a = Some(unsafe { std::mem::transmute_copy::<T, i32>(&field_value) });
					} else {
						panic!("Wrong type")
					}
				}
				1 => {
					if std::any::TypeId::of::<T>() == std::any::TypeId::of::<N>() {
						self.b = Some(unsafe { std::mem::transmute_copy::<T, N>(&field_value) });
					} else {
						panic!("Wrong type")
					}
				}
				_ => panic!("Field not found"),
			}
			self
		}

		fn try_build(self) -> Result<Test<N>, ()> {
			let Some(a) = self.a else { return Err(()) };
			let Some(b) = self.b else { return Err(()) };
			Ok(Test { a, b })
		}
	}
}
#[test]
fn main() {
	let a = Test { a: 32i32, b: 8u8 };
	let b = a.get_field::<u8>("b");
	dbg!(b);

}
