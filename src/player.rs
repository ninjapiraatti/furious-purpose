use bevy::{
	prelude::*,
	prelude::{Input, KeyCode, Res},
};

use crate::{state, game, loading};
use super::{despawn_screen};
use std::collections::HashMap;
use rand::{thread_rng, Rng};

pub struct PlayerPlugin;

#[derive(Component, Debug, Clone)]
struct Player {
	name: String,
}

#[derive(Component, Debug)]
struct PlayerHead {
	direction: Direction,
}

#[derive(Component)]
struct Name(String);

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
	Left,
	Up,
	Right,
	Down,
}

impl Direction {
	fn turn_left(self) -> Self {
		match self {
			Self::Left => Self::Down,
			Self::Right => Self::Up,
			Self::Up => Self::Left,
			Self::Down => Self::Right,
		}
	}
	fn turn_right(self) -> Self {
		match self {
			Self::Left => Self::Up,
			Self::Right => Self::Down,
			Self::Up => Self::Right,
			Self::Down => Self::Left,
		}
	}
}

#[derive(Component, Debug)]
struct PlayerSegment;

#[derive(Resource, Default)]
pub struct PlayerSegments(pub HashMap<String, Vec<Entity>>);


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
			//.init_resource::<Actions>().add_system_set(SystemSet::on_update(state::AppState::Game).with_system(set_movement_actions))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(player_spawn_input))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(player_movement_input))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(move_players))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(grow_player_tails));
	}
}

fn player_spawn_input(
	keyboard_input: Res<Input<KeyCode>>,
	mut in_game_state: ResMut<state::InGameState>,
	mut players: Query<(&Player)>,
	mut commands: Commands,
	textures: Res<loading::TextureAssets>,
) {
	let mut rng = rand::thread_rng();
	if !in_game_state.player1 && keyboard_input.any_just_pressed([KeyCode::Q, KeyCode::W]) {
		println!("SEEING INPUT");
		let start_position = game::Position {
			x: rng.gen_range(100..1100), // Generate random x position between 100 and 1100
			y: rng.gen_range(100..700),  // Generate random y position between 100 and 700
		};
		spawn_player(&mut commands, in_game_state, &textures, "Cookie Crab", start_position, Direction::Down);
	}
}

fn player_movement_input(
	keyboard_input: Res<Input<KeyCode>>,
	mut heads: Query<(&mut PlayerHead, &Player)>,
) {
	for (mut head, player) in heads.iter_mut() {
		let dir: Direction = if player.name == "Cookie Crab" {
			if keyboard_input.just_pressed(KeyCode::Q) {
				Direction::turn_left(head.direction)
			} else if keyboard_input.just_pressed(KeyCode::W) {
				Direction::turn_right(head.direction)
			} else {
				head.direction
			}
		} else if player.name == "Sid Starfish" {
			if keyboard_input.just_pressed(KeyCode::B) {
				Direction::turn_left(head.direction)
			} else if keyboard_input.just_pressed(KeyCode::N) {
				Direction::turn_right(head.direction)
			} else {
				head.direction
			}
		} else if player.name == "Foo Frog" {
			if keyboard_input.just_pressed(KeyCode::O) {
				Direction::turn_left(head.direction)
			} else if keyboard_input.just_pressed(KeyCode::P) {
				Direction::turn_right(head.direction)
			} else {
				head.direction
			}
		} else if player.name == "Jabby Jellyfish" {
			if keyboard_input.just_pressed(KeyCode::Left) {
				Direction::turn_left(head.direction)
			} else if keyboard_input.just_pressed(KeyCode::Right) {
				Direction::turn_right(head.direction)
			} else {
				head.direction
			}
		} else {
				head.direction
		};
		head.direction = dir;
	}
}

/*
fn spawn_players(mut in_game_state: ResMut<state::InGameState>, mut commands: Commands, textures: Res<loading::TextureAssets>) {
	in_game_state.player1 = true;
	in_game_state.player2 = true;
	in_game_state.player3 = true;
	in_game_state.player4 = true;
	let player1_start_position = game::Position { x: 600, y: 450 };
	let player2_start_position = game::Position { x: 650, y: 400 };
	let player3_start_position = game::Position { x: 600, y: 350 };
	let player4_start_position = game::Position { x: 550, y: 400 };

	spawn_player(&mut commands, in_game_state, &textures, "Cookie Crab", player1_start_position, Direction::Up);
	spawn_player(&mut commands, in_game_state, &textures, "Sid Starfish", player2_start_position, Direction::Right);
	spawn_player(&mut commands, in_game_state, &textures, "Foo Frog", player3_start_position, Direction::Down);
	spawn_player(&mut commands, in_game_state, &textures, "Jabby Jellyfish", player4_start_position, Direction::Left);
}
*/


fn spawn_player(
	commands: &mut Commands,
	mut in_game_state: ResMut<state::InGameState>,
	textures: &Res<loading::TextureAssets>,
	player_name: &str,
	start_position: game::Position,
	direction: Direction,
) {
	println!("SPAWNING: {:?}", player_name);
	in_game_state.player1 = true;
	let texture = match player_name {
		"Cookie Crab" => textures.crab.clone(),
		"Sid Starfish" => textures.starfish.clone(),
		"Foo Frog" => textures.frog.clone(),
		"Jabby Jellyfish" => textures.jellyfish.clone(),
		_ => textures.crab.clone()
	};
	commands
		.spawn(SpriteBundle {
			texture: texture,
			transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
			..Default::default()
		})
		.insert(PlayerHead {
			direction: direction,
		})
		.insert(start_position)
		.insert(Player{name: player_name.to_string()});
}


fn get_all_positions(segments: &PlayerSegments, positions: &Query<&mut game::Position>) -> Vec<game::Position> {
	segments.0
		.values()
		.flat_map(|entities| {
			entities
				.iter()
				.flat_map(|entity| positions.get(*entity).ok().map(|pos| *pos))
		})
		.collect()
}

fn move_players(
	mut segments: ResMut<PlayerSegments>,
	mut heads: Query<(Entity, &PlayerHead, &mut Player)>,
	mut positions: Query<&mut game::Position>,
	mut in_game_state: ResMut<state::InGameState>,
	mut commands: Commands,
	textures: Res<loading::TextureAssets>,
) {
	let segment_positions = get_all_positions(&segments, &positions);
	let mut game_over_players = Vec::new();
	for (head_entity, head, mut player) in heads.iter_mut() {
		let mut head_pos = positions.get_mut(head_entity).unwrap();
		//println!("head pos: {:?}", head_pos);
		match head.direction {
			Direction::Left => {
				head_pos.x += -1;
			}
			Direction::Right => {
				head_pos.x += 1;
			}
			Direction::Up => {
				head_pos.y += 1;
			}
			Direction::Down => {
				head_pos.y -= 1;
			}
		}
		if head_pos.x < 0
			|| head_pos.y < 0
			|| head_pos.x as u32 >= game::ARENA_WIDTH
			|| head_pos.y as u32 >= game::ARENA_HEIGHT
			|| segment_positions.contains(&head_pos)
		{
			game_over_players.push(player.name.clone());
			continue;
		}
	}

	for player_name in &game_over_players {
		if let Some(player_segments) = segments.0.remove(player_name) {
			for segment in player_segments {
				commands.entity(segment).despawn();
			}
		}

		if let Some(head_entity) = heads
			.iter()
			.find_map(|(entity, _, player)| {
				if player.name == *player_name {
					Some(entity)
				} else {
					None
				}
			})
		{
			in_game_state.player1 = false;
			commands.entity(head_entity).despawn();
		}
	}
}


fn grow_player_tails(
	mut commands: Commands,
	head_positions: Query<(&game::Position, &Player), With<PlayerHead>>,
	mut segments: ResMut<PlayerSegments>,
) {
	for (head_position, player) in head_positions.iter() {
		let player_segments = segments.0.entry(player.name.clone()).or_insert_with(Vec::new);
		player_segments.push(spawn_segment(&mut commands, head_position.clone(), player.clone()));
	}
}

fn spawn_segment(
	commands: &mut Commands,
	position: game::Position,
	player: Player,
) -> Entity {
	let color = match &player.name as &str{
		"Cookie Crab" => Color::rgb(0.99, 0.45, 0.0),
		"Sid Starfish" => Color::rgb(0.99, 0.25, 0.20),
		"Foo Frog" => Color::rgb(0.5, 0.95, 0.3),
		"Jabby Jellyfish" => Color::rgb(0.3, 0.75, 0.99),
		_ => Color::rgb(0.45, 0.0, 0.99), // Default color
	};
	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: color,
				custom_size: Some(Vec2::new(3.0, 3.0)),
				..default()
			},
					..default()
		})
		.insert(position)
		.insert(player)
		.id()
}