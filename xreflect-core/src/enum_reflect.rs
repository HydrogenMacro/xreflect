use crate::types::{ReflectError, StructLikeType};

pub trait EnumReflect: Sized {
	const MEMBERS: &'static [StructLikeType];
	fn reflect_type_name_of_member(_member: &str) -> &'static str {
		todo!()
	}
	fn construct(_enum_member: &str) -> Self {
		todo!()
	}
	fn get_field<T>(&self, _field_name: &str) -> &T {
		todo!()
	}
	fn set_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn has_field(&self, _field_name: &str) -> bool { todo!() }
	fn with_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Self {
		todo!()
	}
	fn get_element<T>(&self, _element_index: usize) -> &T {
		todo!()
	}
	fn set_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn has_element(&self, _element_index: usize) -> bool { todo!() }
	fn with_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Self {
		todo!()
	}
}

pub trait EnumBuilder: EnumReflect + Default {

}