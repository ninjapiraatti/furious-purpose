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
		//println!("Player: {:?}, dir: {:?}", player.name, dir);
        head.direction = dir;
    }
}


fn spawn_players(mut commands: Commands, textures: Res<loading::TextureAssets>) {
	let player1_start_position = game::Position { x: 600, y: 450 };
	let player2_start_position = game::Position { x: 650, y: 400 };
	let player3_start_position = game::Position { x: 600, y: 350 };
	let player4_start_position = game::Position { x: 550, y: 400 };

	spawn_player(&mut commands, &textures, "Cookie Crab", player1_start_position, Direction::Up);
	spawn_player(&mut commands, &textures, "Sid Starfish", player2_start_position, Direction::Right);
	spawn_player(&mut commands, &textures, "Foo Frog", player3_start_position, Direction::Down);
	spawn_player(&mut commands, &textures, "Jabby Jellyfish", player4_start_position, Direction::Left);
}


fn spawn_player(
	commands: &mut Commands,
	textures: &Res<loading::TextureAssets>,
	player_name: &str,
	start_position: game::Position,
	direction: Direction,
) {
	println!("SPAWNING: {:?}", player_name);
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
	mut heads: Query<(Entity, &PlayerHead, &Player)>,
	mut positions: Query<&mut game::Position>,
	mut commands: Commands,
	textures: Res<loading::TextureAssets>,
) {
	let segment_positions = get_all_positions(&segments, &positions);
	let mut game_over_players = Vec::new();
	for (head_entity, head, player) in heads.iter_mut() {
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
			commands.entity(head_entity).despawn();
		}
		let start_position = game::Position { x: 600, y: 400 };
		spawn_player(&mut commands, &textures, player_name, start_position, Direction::Right);
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