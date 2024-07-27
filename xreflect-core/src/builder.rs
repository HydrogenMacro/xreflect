pub trait EnumBuildable<B: Builder<Self>>: Sized {
	fn build(enum_member_name: &'static str) -> B;
}
pub trait StructBuildable<B: Builder<Self>>: Sized {
	fn build() -> B;
}
pub trait Builder<B: Sized>: Sized {
	fn new() -> Self;
	fn with_field<T: 'static>(&mut self, field_name: &'static str, field_value: T);
	fn with_field_at<T: 'static>(&mut self, field_index: usize, field_value: T);
	fn try_build(self) -> Result<B, ()>;
	fn build(self) -> B {
		self.try_build().unwrap()
	}
}