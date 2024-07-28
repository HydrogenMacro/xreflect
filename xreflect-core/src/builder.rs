pub trait EnumBuildable: Sized {
	fn build(enum_member_name: &'static str) -> impl Builder<Self>;
}
pub trait StructBuildable: Sized {
	fn build() -> impl Builder<Self>;
}
pub trait Builder<B: Sized>: Sized {
	fn with_field<T>(self, field_name: &'static str, field_value: T) -> Self;
	fn with_field_at<T>(self, field_index: usize, field_value: T) -> Self;
	fn try_build(self) -> Result<B, ()>;
	fn build(self) -> B {
		self.try_build().unwrap()
	}
}