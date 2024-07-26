use std::mem::MaybeUninit;
use crate::types::{ReflectError, StructLikeData};

pub trait EnumReflectInternal: Sized {
	const MEMBERS: &'static [StructLikeData];
	const MEMBER_NAMES: &'static [&'static str];
	const TYPE_PATH: &'static str;
	const TYPE_NAME: &'static str;
	fn get_index_of_member_name(member_name: &str) -> Result<usize, ()>;
	fn create_member_from_raw_parts(member_name: &str, member_data: StructLikeData);
	fn get_field_from_index<T>(&self, index: usize) -> Result<&T, ()>;
	fn get_field_from_index_mut<T>(&mut self, index: usize) -> Result<&mut T, ()>;
}
pub trait EnumReflect: EnumReflectInternal {
	fn member_names() -> &'static [&'static str] {
		Self::MEMBER_NAMES
	}
	fn get_field<T>(&self, field_name: &str) -> &T {
		let i = Self::get_index_of_member_name(field_name)
			.expect(&format!("Field '{}' does not exist on ", field_name));
		self.get_field_from_index::<T>(i).unwrap()
	}
	fn get_field_mut<T>(&mut self, field_name: &str) -> &mut T;
	fn set_field<T>(&mut self, field_name: &str, new_field_value: T) -> Result<(), ReflectError>;
	fn has_field(&self, field_name: &str) -> bool;
	fn get_field_at<T>(&self, field_index: usize) -> &T;
	fn get_field_mut_at<T>(&mut self, field_index: usize) -> &mut T;
	fn set_field_at<T>(&mut self, field_index: usize, new_field_value: T) -> Result<(), ReflectError>;
	fn has_field_at(&self, field_index: usize) -> bool;
	fn construct(member_name: &'static str) -> impl EnumBuilder<Self>;
}
pub trait EnumBuilder<E: EnumReflect>: Sized {
	fn new(enum_member_name: &'static str) -> Self;
	fn with_field<T>(&mut self, field_name: &'static str, _field_value: T);
	fn with_field_at<T>(&mut self, field_index: usize, _field_value: T);
	fn try_finalize<T>(&self) -> Result<E, ()>;
	fn finalize(&self) -> E;
}
