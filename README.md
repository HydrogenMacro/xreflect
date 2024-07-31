# XReflect
A reflection library

## Quickstart
```rust
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
	// You can now access and modify fields dynamically
	let mut enemy = Enemy { health: 2 };

	let enemy_health = enemy.get_field::<u8>("health");
	assert_eq!(enemy_health, &2);
	enemy.set_field::<u8>("health", 7);
	assert_eq!(enemy_health, &7);
}
```