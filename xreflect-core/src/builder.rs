pub trait EnumBuildable: Sized {
	fn build() -> impl EnumBuilder<Self>;
}
pub trait StructBuildable: Sized {
	fn build() -> impl StructBuilder<Self>;
}
pub trait Builder<B: Sized> {
	fn with_field<T: 'static>(&mut self, field_name: &'static str, _field_value: T);
	fn with_field_at<T: 'static>(&mut self, field_index: usize, _field_value: T);
	fn try_build(&self) -> Result<B, ()>;
	fn build(&self) -> B;
}
pub trait EnumBuilder<B: Sized>: Builder<B> + Sized {
	fn new(enum_member_name: &'static str) -> Self;
	fn try_new(enum_member_name: &'static str) -> Result<Self, ()>;
}
pub trait StructBuilder<B: Sized>: Builder<B> + Sized {
	fn new() -> Self;
	fn try_new() -> Result<Self, ()>;
}