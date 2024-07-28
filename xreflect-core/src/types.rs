use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

#[derive(Debug)]
pub enum StructType {
	Unit,
	Record(&'static [(&'static str, TypeId)]),
	Tuple(&'static [TypeId])
}

#[derive(Debug)]
pub enum ReflectError {
	EnumMemberNotFound,
	FieldNotFound,
	WrongType,
}
