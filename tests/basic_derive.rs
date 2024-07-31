use xreflect::ReflectError;
use xreflect::Reflect;
#[derive(Reflect, PartialEq)]
struct TestStruct {
	test_struct_field_a: i32,
	test_struct_field_b: u8,
}

#[derive(Reflect, PartialEq)]
enum TestEnum {
	Unit,
	Tuple(i32),
	Struct { test_enum_field: u8, test_enum_field_2: i16 },
}

#[test]
fn main() {
	let unit = TestEnum::Unit;
	assert_eq!(unit.amount_of_fields(), 0);
	assert_eq!(unit.try_get_field_at::<i32>(0), Err(ReflectError::FieldNotFound));
	let tuple = TestEnum::Tuple(33);
	assert_eq!(tuple.amount_of_fields(), 1);
	assert_eq!(tuple.try_get_field_at::<i32>(0), Ok(&33));
	let test_struct = TestEnum::Struct { test_enum_field: 71, test_enum_field_2: -21 };
	assert_eq!(test_struct.amount_of_fields(), 2);
	assert_eq!(test_struct.try_get_field::<u8>("test_enum_field"), Ok(&71u8));
}
