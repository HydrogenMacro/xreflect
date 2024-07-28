use crate::{ReflectError, StructType};

pub trait Reflect: Sized {
	fn amount_of_fields(&self) -> usize;

	fn try_get_field_at<T: 'static>(&self, field_index: usize) -> Result<&T, ReflectError>;
	fn try_get_field_mut_at<T: 'static>(&mut self, field_index: usize) -> Result<&mut T, ReflectError>;
	fn try_set_field_at<T: 'static>(&mut self, field_index: usize, new_field_value: T) -> Result<(), ReflectError>;
	fn try_get_type_of_field_at(&self, field_index: usize) -> Result<std::any::TypeId, ReflectError>;

	fn has_field(&self, field_name: &str) -> bool {
		self.get_index_of_field(field_name).is_ok()
	}
	fn has_field_at(&self, field_index: usize) -> bool {
		field_index < self.amount_of_fields()
	}

}

pub trait StructReflect {
	const TYPE: StructType;
	const FIELD_NAMES: &'static [&'static str];
}

pub trait EnumReflect {
	const MEMBER_NAMES: &'static [&'static str];
	const MEMBER_TYPES: &'static [StructType];
	fn get_index_of_member(member_name: &str) -> Option<usize>;
}