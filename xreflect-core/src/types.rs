#[derive(Debug)]
pub enum StructLikeData {
	Unit,
	Record(&'static [Option<TypePath>]),
	Tuple(&'static [Option<TypePath>])
}

#[derive(Debug)]
pub enum ReflectError {
	EnumMemberNotFound,
	FieldNotFound,
	WrongType,
}
#[derive(Debug)]
pub struct TypePath(&'static str);
impl TypePath {
	pub const fn new(path: &'static str) -> TypePath {
		TypePath(path)
	}
}