/*
use xreflect::Reflect;
// just derive Reflect
#[derive(Reflect)]
pub struct Enemy {
	pub health: u8,
}
#[derive(Reflect)]
pub enum GameState {
	Playing,
	Won { score: i32, remaining_health: u8 },
	Lost(Enemy),
}

fn main() {
	// With Reflect, you can:

	// 1. access and modify fields dynamically
	let mut enemy = Enemy { health: 0 };

	let enemy_health = enemy.field::<u8>("health");
	enemy.set_field::<u8>("health", 7);

	// 2. construct items dynamically
	let mut game_state = GameState::Playing;

	game_state = GameState::construct("Won")
		.with_field::<f32>("score", 0.0)
		.with_field::<u8>("remaining_health", 20);

	game_state = GameState::construct("Lost").with_tuple_field::<Enemy>(0, enemy);

	// 3. iterate through an item's fields and their types
	for (field_name, field_type) in Enemy::fields() {
		// both field_name and field_type are &str
		match field_type {
			"i32" => {
				// note that you still have to provide the generic with the actual type
				enemy.field::<i32>(field_name) = 1;
			}
			// custom types have convenience associated functions
			GameState::type_name() => {}
		}
	}

	#[derive(Default, Reflect)]
	pub struct Foo {
		// pub fields are reflected by default
		pub reflected: (),

		// pub(crate)/pub(super)/private fields aren't
		unreflected_by_default: (),

		// but you can opt in (with an alias)
		#[reflect(true, "cool_new_alias")]
		reflected_even_if_private: (),

		// or opt out
		#[reflect(false)]
		pub unreflected_even_when_pub: (),
	}
}
*/
fn main() {}