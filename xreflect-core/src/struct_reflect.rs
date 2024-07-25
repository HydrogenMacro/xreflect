
pub trait StructReflect: Sized {
	const STRUCT_TYPE: ;
	fn get_type_name() ->  {
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