use bevy::{
	prelude::*,
	prelude::{Input, KeyCode, Res},
};

use crate::{state, loading::TextureAssets};
use super::{despawn_screen};

pub struct PlayerPlugin;

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

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `state::AppState::Game`
impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(spawn_player))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(player_input));
	}
}

fn player_input(
	keyboard_input: Res<Input<KeyCode>>,
	//mut query: Query<&mut Transform>
) {
	if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
		println!("UP");
	}
	if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
		println!("DOWN");
	}
	if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
		println!("LEFT");
	}
	if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
		println!("RIGHT");
	}
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
	commands
		.spawn(SpriteBundle {
			texture: textures.logo.clone(),
			transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
			..Default::default()
		})
		.insert(Player);
}

/*
fn move_player(
	time: Res<Time>,
	actions: Res<Actions>,
	mut player_query: Query<&mut Transform, With<Player>>,
) {
	if actions.player_movement.is_none() {
		return;
	}
	let speed = 150.;
	let movement = Vec3::new(
		actions.player_movement.unwrap().x * speed * time.delta_seconds(),
		actions.player_movement.unwrap().y * speed * time.delta_seconds(),
		0.,
	);
	for mut player_transform in &mut player_query {
		player_transform.translation += movement;
	}
}
*/