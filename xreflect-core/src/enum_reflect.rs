use std::mem::MaybeUninit;
use crate::types::{ArbitraryTypeName, ReflectError, StructLikeData};

trait EnumReflectInternal: Sized {
	const MEMBERS: &'static [StructLikeData];
	fn get_member_info(_member_name: &'static str) -> StructLikeData;
	fn get_index_of_member_name(_member_name: &'static str) -> usize;
	fn create_enum_from_raw_parts(_member_name: &'static str, _member_data: StructLikeData);
}
pub trait EnumReflect: EnumReflectInternal {
	fn reflect_type_name_of_member(_member: &str) -> &'static str;
	fn get_field<T>(&self, _field_name: &str) -> &T;
	fn get_field_mut<T>(&mut self, _field_name: &str) -> &mut T;
	fn set_field<T>(&mut self, _field_name: &str, _new_field_value: T) -> Result<(), ReflectError>;
	fn has_field(&self, _field_name: &str) -> bool;
	fn get_field_at<T>(&self, _field_index: usize) -> &T;
	fn get_field_mut_at<T>(&mut self, _field_index: usize) -> &mut T;
	fn set_field_at<T>(&mut self, _field_index: usize, _new_field_value: T) -> Result<(), ReflectError>;
	fn has_field_at(&self, _field_index: usize) -> bool;
	fn construct(_member_name: &'static str) -> impl EnumBuilder;
}
pub trait EnumBuilder: Sized {
	fn new() -> Self;
	fn with_field<T>(&mut self, _field_name: &'static str, _field_value: T);
	fn with_field_at<T>(&mut self, _field_index: usize, _field_value: T);
	fn try_finalize<T>(&self) -> Result<(), ()>;
	fn finalize(&self);
}
