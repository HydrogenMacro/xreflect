pub type BasicTypeName = &'static str;
pub type ArbitraryTypeName = Option<ArbitraryTypeName>;

pub enum StructLikeData {
	Unit,
	Record(&'static [(&'static str, ArbitraryTypeName)]),
	Tuple(&'static [ArbitraryTypeName])
}
pub enum ReflectError {
	FieldNotFound,
	WrongType,
	IsNotTupleLike,
	IsNotRecordLike,
}