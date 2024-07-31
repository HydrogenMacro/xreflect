use std::{
	any::{TypeId},

};

#[derive(Debug, PartialEq, Eq)]
pub enum StructType {
	Unit,
	Record(&'static [(&'static str, TypeId)]),
	Tuple(&'static [TypeId]),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReflectError {
	EnumMemberNotFound,
	FieldNotFound,
	WrongType,
}
