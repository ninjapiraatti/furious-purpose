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

enum GameControl {
	Left,
	Up,
	Right,
	Down,
}

impl GameControl {
	fn opposite(self) -> Self {
		match self {
			Self::Left => Self::Right,
			Self::Right => Self::Left,
			Self::Up => Self::Down,
			Self::Down => Self::Up,
		}
	}
}

struct PlayerHead {
	direction: Direction,
}

#[derive(Default, Resource)]
struct Actions {
	pub player_movement: Option<Vec2>,
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
			.init_resource::<Actions>().add_system_set(SystemSet::on_update(state::AppState::Game).with_system(set_movement_actions))
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(spawn_player))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(move_player));
	}
}

impl GameControl {
	pub fn pressed(
		&self,
		keyboard_input: &Res<Input<KeyCode>>,
		//mut query: Query<&mut Transform>
	) -> bool {
		match self {
			GameControl::Up => {
				keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
			}
			GameControl::Down => {
				keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
			}
			GameControl::Left => {
				keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
			}
			GameControl::Right => {
				keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
			}
		}
	}
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
	let player_movement = Vec2::new(
		get_movement(GameControl::Right, &keyboard_input)
			- get_movement(GameControl::Left, &keyboard_input),
		get_movement(GameControl::Up, &keyboard_input)
			- get_movement(GameControl::Down, &keyboard_input),
	);

	if player_movement != Vec2::ZERO {
		actions.player_movement = Some(player_movement.normalize());
	} else {
		actions.player_movement = None;
	}
}

fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>) -> f32 {
	if control.pressed(input) {
		1.0
	} else {
		0.0
	}
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
	commands
		.spawn(SpriteBundle {
			texture: textures.crab.clone(),
			transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
			..Default::default()
		})
		.insert(Player);
}

fn move_player(
	time: Res<Time>,
	actions: Res<Actions>,
	mut player_query: Query<&mut Transform, With<Player>>,
) {
	if actions.player_movement.is_none() {
		return;
	}
	let speed = 150.0;
	let movement = Vec3::new(
		actions.player_movement.unwrap().x * speed * time.delta_seconds(),
		actions.player_movement.unwrap().y * speed * time.delta_seconds(),
		0.,
	);
	for mut player_transform in &mut player_query {
		println!("Movement: {:?}", movement);
		player_transform.translation += movement;
	}
}