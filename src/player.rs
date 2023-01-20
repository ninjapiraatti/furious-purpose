use bevy::prelude::*;
pub struct HelloPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

enum Direction {
	Left,
	Up,
	Right,
	Down,
}

struct PlayerHead {
	direction: Direction,
}

impl Direction {
	fn opposite(self) -> Self {
		match self {
			Self::Left => Self::Right,
			Self::Right => Self::Left,
			Self::Up => Self::Down,
			Self::Down => Self::Up,
		}
	}
}

pub enum PlayerMovement {
	Input,
	Movement,
	Growth,
	Spawn,
}

fn add_people(mut commands: Commands) {
	commands.spawn((Player, Name("Elaina Proctor".to_string())));
	commands.spawn((Player, Name("Renzo Hume".to_string())));
	commands.spawn((Player, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Player>>) {
	for name in query.iter() {
		println!("hello {}!", name.0);
	}
}

impl Plugin for HelloPlugin {
	fn build(&self, app: &mut App) {
		app.add_startup_system(add_people)
		.add_system(greet_people);
	}
}

pub fn lol() {
	println!("lol");
}