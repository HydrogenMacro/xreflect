use phf;
pub trait StructReflect: Sized {
	const STRUCT_TYPE: StructType;
	fn type_name() -> &'static str {
		todo!()
	}
	fn get_field<T>(&self, _field_name: &str) -> &T {
		todo!()
	}
	fn set_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Self {
		todo!()
	}
	fn get_element<T>(&self, _element_index: usize) -> &T {
		todo!()
	}
	fn set_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Result<(), ReflectError> {
		todo!()
	}
	fn with_element<T>(&mut self, _element_index: usize, _new_field_value: T) -> Self {
		todo!()
	}
}
pub trait EnumReflect: Sized {
	const MEMBERS: phf::Map<&'static str, StructType>;
	fn type_name_of_member(_member: &str) -> &'static str {
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
pub enum StructType {
	Unit,
	Record(phf::Map<&'static str, &'static str>),
	Tuple(Vec<&'static str>)
}
pub enum ReflectError {
	FieldNotFound,
	WrongType,
	IsNotTupleLike,
	IsNotRecordLike,
}