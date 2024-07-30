use xreflect::ReflectError;
use xreflect::Reflect;

#[derive(Reflect)]
enum Test {
	Unit,
	Tuple(i32),
	Struct { field: u8 },
}

#[test]
fn main() {
	let a = Test::Tuple(36);
	let b = a.get_field::<i32>("0");
	dbg!(b);
}
