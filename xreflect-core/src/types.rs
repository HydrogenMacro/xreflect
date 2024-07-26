type TypePath = &'static str;

#[derive(Debug)]
pub enum StructLikeData {
	Unit,
	Record(&'static [(&'static str, Option<TypePath>)]),
	Tuple(&'static [Option<TypePath>])
}

#[derive(Debug)]
pub enum ReflectError {
	FieldNotFound,
	WrongType,
	IsNotTupleLike,
	IsNotRecordLike,
}
