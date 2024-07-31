use std::{
	any::{type_name, type_name_of_val, TypeId},
};
use crate::{ReflectError, StructType};
pub trait Reflect: Sized {
	fn amount_of_fields(&self) -> usize;

	fn try_get_index_of_field(&self, field_name: &str) -> Result<usize, ReflectError>;
	fn get_index_of_field(&self, field_name: &str) -> usize {
		self.try_get_index_of_field(field_name).expect(&format!(
			"There is no field '{}' in {}",
			field_name,
			type_name::<Self>()
		))
	}
	fn try_get_field_at<T: 'static>(&self, field_index: usize) -> Result<&T, ReflectError>;
	fn try_get_field_mut_at<T: 'static>(
		&mut self,
		field_index: usize,
	) -> Result<&mut T, ReflectError>;
	fn try_set_field_at<T: 'static>(
		&mut self,
		field_index: usize,
		new_field_value: T,
	) -> Result<(), ReflectError> {
		let field_mut = self.try_get_field_mut_at(field_index)?;
		*field_mut = new_field_value;
		Ok(())
	}
	fn try_get_type_of_field_at(&self, field_index: usize) -> Result<TypeId, ReflectError>;

	fn get_field_at<T: 'static>(&self, field_index: usize) -> &T {
		match self.try_get_field_at(field_index) {
			Ok(field_value) => field_value,
			Err(ReflectError::WrongType) => panic!(
				"The type of the field at {} is not {} in {}",
				field_index,
				type_name::<T>(),
				type_name_of_val(self)
			),
			Err(ReflectError::FieldNotFound) => panic!(
				"There is no field at {} in {}",
				field_index,
				type_name_of_val(self)
			),
			Err(ReflectError::EnumMemberNotFound) => unreachable!(),
		}
	}
	fn get_field_mut_at<T: 'static>(&mut self, field_index: usize) -> &mut T {
		let type_name_of_self = type_name_of_val(self);
		match self.try_get_field_mut_at(field_index) {
			Ok(field_value) => field_value,
			Err(ReflectError::WrongType) => panic!(
				"The type of the field at {} is not {} in {}",
				field_index,
				type_name::<T>(),
				type_name_of_self
			),
			Err(ReflectError::FieldNotFound) => panic!(
				"There is no field at {} in {}",
				field_index, type_name_of_self
			),
			Err(ReflectError::EnumMemberNotFound) => unreachable!(),
		}
	}
	fn set_field_at<T: 'static>(&mut self, field_index: usize, new_field_value: T) {
		match self.try_set_field_at(field_index, new_field_value) {
			Ok(()) => (),
			Err(ReflectError::WrongType) => panic!(
				"The type of the field at {} is not {} in {}",
				field_index,
				type_name::<T>(),
				type_name_of_val(self)
			),
			Err(ReflectError::FieldNotFound) => panic!(
				"There is no field at {} in {}",
				field_index,
				type_name_of_val(self)
			),
			Err(ReflectError::EnumMemberNotFound) => unreachable!(),
		}
	}
	fn get_type_of_field_at(&self, field_index: usize) -> TypeId {
		match self.try_get_type_of_field_at(field_index) {
			Ok(type_id) => type_id,
			Err(ReflectError::WrongType) => unreachable!(),
			Err(ReflectError::FieldNotFound) => panic!(
				"There is no field at {} in {}",
				field_index,
				type_name_of_val(self)
			),
			Err(ReflectError::EnumMemberNotFound) => unreachable!(),
		}
	}

	fn try_get_field<T: 'static>(&self, field_name: &'static str) -> Result<&T, ReflectError> {
		let field_index = self.try_get_index_of_field(field_name)?;
		self.try_get_field_at(field_index)
	}
	fn try_get_field_mut<T: 'static>(
		&mut self,
		field_name: &'static str,
	) -> Result<&mut T, ReflectError> {
		let field_index = self.try_get_index_of_field(field_name)?;
		self.try_get_field_mut_at(field_index)
	}
	fn try_set_field<T: 'static>(
		&mut self,
		field_name: &'static str,
		new_field_value: T,
	) -> Result<(), ReflectError> {
		let field_index = self.try_get_index_of_field(field_name)?;
		self.try_set_field_at(field_index, new_field_value)
	}
	fn try_get_type_of_field(&self, field_name: &'static str) -> Result<TypeId, ReflectError> {
		let field_index = self.try_get_index_of_field(field_name)?;
		self.try_get_type_of_field_at(field_index)
	}
	fn get_field<T: 'static>(&self, field_name: &'static str) -> &T {
		let field_index = self.get_index_of_field(field_name);
		self.get_field_at(field_index)
	}
	fn get_field_mut<T: 'static>(&mut self, field_name: &'static str) -> &mut T {
		let field_index = self.get_index_of_field(field_name);
		self.get_field_mut_at(field_index)
	}
	fn set_field<T: 'static>(&mut self, field_name: &'static str, new_field_value: T) -> () {
		let field_index = self.get_index_of_field(field_name);
		self.set_field_at(field_index, new_field_value)
	}
	fn get_type_of_field(&self, field_name: &'static str) -> TypeId {
		let field_index = self.get_index_of_field(field_name);
		self.get_type_of_field_at(field_index)
	}

	fn has_field(&self, field_name: &str) -> bool {
		self.try_get_index_of_field(field_name).is_ok()
	}
	fn has_field_at(&self, field_index: usize) -> bool {
		field_index < self.amount_of_fields()
	}
}

pub trait StructReflect {
	const TYPE: StructType;
	const FIELD_NAMES: &'static [&'static str];
	fn try_get_index_of_field(field_name: &str) -> Result<usize, ReflectError>;
	fn get_index_of_field(field_name: &str) -> usize {
		Self::try_get_index_of_field(field_name).expect(&format!(
			"There is no field '{}' in {}",
			field_name,
			type_name::<Self>()
		))
	}
}

pub trait EnumReflect {
	const MEMBER_NAMES: &'static [&'static str];
	const MEMBER_TYPES: &'static [StructType];
	fn get_index_of_member(member_name: &str) -> Result<usize, ReflectError>;
}
