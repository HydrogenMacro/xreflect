use std::{
	any::{Any, TypeId},
	cell::UnsafeCell,
	collections::HashSet,
	sync::{LazyLock, Mutex},
};

#[derive(Debug)]
pub enum StructType {
	Unit,
	Record(&'static [(&'static str, TypeId)]),
	Tuple(&'static [TypeId]),
}

#[derive(Debug)]
pub enum ReflectError {
	EnumMemberNotFound,
	FieldNotFound,
	WrongType,
}
