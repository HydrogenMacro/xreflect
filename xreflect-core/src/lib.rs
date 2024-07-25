use std::any::Any;
use phf;
pub trait StructReflect: Sized {
	fn type_name() -> &'static str {
		todo!()
	}
	fn list_fields() -> impl Iterator<Item=(&'static str, &'static str)> {
		todo!()
	}
	fn list_elements() -> impl Iterator<Item=&'static str> {
		todo!()
	}
	fn get_field<T>(&self, field_name: &str) -> &T {
		todo!()
	}
	fn set_field<T>(&mut self, field_name: &str, new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_field<T>(&mut self, field_name: &str, new_field_value: T) -> Self {
		todo!()
	}
	fn get_element<T>(&self, element_index: usize) -> &T {
		todo!()
	}
	fn set_element<T>(&mut self, element_index: usize, new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_element<T>(&mut self, element_index: usize, new_field_value: T) -> Self {
		todo!()
	}
}
pub trait EnumReflect: Sized {
	const AMOUNT_OF_OPTIONS: usize;
	fn members() -> [&'static str; Self::AMOUNT_OF_OPTIONS] {
		todo!()
	}
	fn type_name_of_member(member: &str) -> &'static str {
		todo!()
	}
	fn list_fields_for_member(member: &str) -> impl Iterator<Item=(String, String)> {
		todo!()
	}
	fn construct(enum_member: &str) -> Self {
		todo!()
	}
	fn get_field<T>(&self, field_name: &str) -> &T {
		todo!()
	}
	fn set_field<T>(&mut self, field_name: &str, new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_field<T>(&mut self, field_name: &str, new_field_value: T) -> Self {
		todo!()
	}
	fn element<T>(&self, element_index: usize) -> &T {
		todo!()
	}
	fn set_element<T>(&mut self, element_index: usize, new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_element<T>(&mut self, element_index: usize, new_field_value: T) -> Self {
		todo!()
	}
}

pub enum ReflectError {
	FieldNotFound,
	WrongType,
	IsNotTupleLike,
	IsNotRecordLike,
}