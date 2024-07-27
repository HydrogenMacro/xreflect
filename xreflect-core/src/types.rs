#[derive(Debug)]
pub enum StructLikeData {
	Unit,
	Record(&'static [Option<TypePath>]),
	Tuple(&'static [Option<TypePath>])
}

#[derive(Debug)]
pub enum ReflectError {
	FieldNotFound,
	WrongType,
	IsNotTupleLike,
	IsNotRecordLike,
}
#[derive(Debug)]
pub struct TypePath(&'static str);
impl TypePath {
	pub fn new(path: &'static str) -> TypePath {
		TypePath(path)
	}
}