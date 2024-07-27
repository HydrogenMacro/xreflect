use crate::types::{ReflectError, StructLikeData};

pub trait EnumReflectInternal: Sized {
	const MEMBERS: &'static [StructLikeData];
	const MEMBER_NAMES: &'static [&'static str];
	const TYPE_PATH: &'static str;
	const TYPE_NAME: &'static str;
	fn get_index_of_member_name(member_name: &str) -> Result<usize, ReflectError>;
	fn get_field_at_index<T: 'static>(&self, index: usize) -> Result<&T, ReflectError>;
	fn get_field_at_index_mut<T: 'static>(&mut self, index: usize) -> Result<&mut T, ReflectError>;
}
pub trait EnumReflect: EnumReflectInternal {
	fn member_names() -> &'static [&'static str] {
		Self::MEMBER_NAMES
	}
	fn get_field<T: 'static>(&self, field_name: &str) -> &T {
		let i = Self::get_index_of_member_name(field_name)
			.expect(&format!("Field '{}' does not exist on {}", field_name, Self::TYPE_NAME));
		self.get_field_at_index::<T>(i).unwrap()
	}
	fn get_field_mut<T: 'static>(&mut self, field_name: &str) -> &mut T {
		let i = Self::get_index_of_member_name(field_name)
			.expect(&format!("Field '{}' does not exist on {}", field_name, Self::TYPE_NAME));
		self.get_field_at_index_mut::<T>(i).unwrap()
	}
	fn set_field<T: 'static>(&mut self, field_name: &str, new_field_value: T) -> Result<(), ReflectError> {
		let Ok(i) = Self::get_index_of_member_name(field_name) else {
			return Err(ReflectError::FieldNotFound);
		};
		let Ok(field_ref) = self.get_field_at_index_mut::<T>(i) else {
			return Err(ReflectError::WrongType);
		};
		*field_ref = new_field_value;
		Ok(())
	}
	fn has_field(&self, field_name: &str) -> bool {
		Self::get_index_of_member_name(field_name).is_ok()
	}
	fn get_field_at<T: 'static>(&self, field_index: usize) -> &T {
		self.get_field_at_index::<T>(field_index).unwrap()
	}
	fn get_field_mut_at<T: 'static>(&mut self, field_index: usize) -> &mut T {
		self.get_field_at_index_mut::<T>(field_index).unwrap()
	}
	fn set_field_at<T: 'static>(&mut self, field_index: usize, new_field_value: T) -> Result<(), ReflectError> {
		let Ok(field_ref) = self.get_field_at_index_mut::<T>(field_index) else {
			return Err(ReflectError::WrongType);
		};
		*field_ref = new_field_value;
		Ok(())
	}
	fn has_field_at(&self, field_index: usize) -> bool {
		field_index < Self::MEMBERS.len()
	}
}
