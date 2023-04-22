use bevy::{
	prelude::*,
	prelude::{Input, KeyCode, Res},
};

use crate::{state, game, loading};
use super::{despawn_screen};
use std::collections::HashMap;

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
//pub struct PlayerSegments(Vec<Entity>);
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
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(spawn_players))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(player_movement_input))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(move_players))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(grow_player_tails));
	}
}

// Move player
fn player_movement_input(
	keyboard_input: Res<Input<KeyCode>>,
	mut heads: Query<&mut PlayerHead>,
	//mut game_state: ResMut<State<state::AppState>>,
) {
	if let Some(mut head) = heads.iter_mut().next() {
		let dir: Direction = if keyboard_input.just_pressed(KeyCode::Left) {
			println!("Turned left");
			Direction::turn_left(head.direction)
		} else if keyboard_input.just_pressed(KeyCode::Right) {
			println!("Turned right");
			Direction::turn_right(head.direction)
		} else {
			head.direction
		};
		head.direction = dir;
	}
}

fn spawn_players(mut commands: Commands, textures: Res<loading::TextureAssets>) {
	let player1_start_position = game::Position { x: 100, y: 100 };
	let player2_start_position = game::Position { x: 200, y: 200 };

	spawn_player(&mut commands, &textures, "ninjapiraatti", player1_start_position);
	spawn_player(&mut commands, &textures, "player2", player2_start_position);
}


fn spawn_player(
	commands: &mut Commands,
	textures: &Res<loading::TextureAssets>,
	player_name: &str,
	start_position: game::Position,
) {
	println!("{:?}", player_name);
	commands
		.spawn(SpriteBundle {
			texture: textures.crab.clone(),
			transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
			..Default::default()
		})
		.insert(PlayerHead {
			direction: Direction::Up,
		})
		.insert(start_position)
		.insert(Player{name: player_name.to_string()});
}

/*
fn move_players(
	mut segments: ResMut<PlayerSegments>,
	mut heads: Query<(Entity, &PlayerHead)>,
	//mut players: Query<(Entity, &Player)>,
	mut positions: Query<&mut game::Position>,
	mut positions2: Query<&mut Transform, With<Player>>,
) {
	if let Some((head_entity, head)) = heads.iter_mut().next() {
		let segment_positions = segments
			.0
			.iter()
			.map(|e| *positions.get_mut(*e).unwrap())
			.collect::<Vec<game::Position>>();
		let mut head_pos = positions.get_mut(head_entity).unwrap();
		println!("head pos: {:?}", head_pos);
		if head_pos.x < 0
			|| head_pos.y < 0
			|| head_pos.x as u32 >= game::ARENA_WIDTH
			|| head_pos.y as u32 >= game::ARENA_HEIGHT
		{
			println!("GAME OVER");
		}
		match &head.direction {
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
		if segment_positions.contains(&head_pos) {
			println!("GAME OVER");
		}
	}
}
*/

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
	mut heads: Query<(Entity, &PlayerHead, &Player)>,
	mut positions: Query<&mut game::Position>,
) {
	let segment_positions = get_all_positions(&segments, &positions);
	for (head_entity, head, player) in heads.iter_mut() {
		//println!("FOUND {:?}{:?}", segment_entities, player.name);
		let mut head_pos = positions.get_mut(head_entity).unwrap();
		println!("head pos: {:?}", head_pos);
		if head_pos.x < 0
			|| head_pos.y < 0
			|| head_pos.x as u32 >= game::ARENA_WIDTH
			|| head_pos.y as u32 >= game::ARENA_HEIGHT
		{
			println!("GAME OVER");
		}
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
		if segment_positions.contains(&head_pos) {
			println!("GAME OVER");
		}
	}
}

fn grow_player_tails(
	mut commands: Commands,
	head_positions: Query<(&game::Position, &Player), With<PlayerHead>>,
	//head_positions: Query<&game::Position, With<PlayerHead>>,
	mut segments: ResMut<PlayerSegments>,
	//materials: Res<loading::Materials>,
) {
	//println!("head_positions: {:?}", head_positions);
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
	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: Color::rgb(0.99, 0.45, 0.0),
				custom_size: Some(Vec2::new(3.0, 3.0)),
				..default()
			},
        	..default()
		})
		/*
		.insert(PlayerSegment{
			name: player_name.to_string()
			position: position
		)
		*/
		.insert(position)
		.insert(player)
		.id()
}