use bevy::{
	prelude::*,
	prelude::{Input, KeyCode, Res},
};

use crate::{state, game, loading};
use super::{despawn_screen};

pub struct PlayerPlugin;

#[derive(Component, Debug)]
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
pub struct PlayerSegments(Vec<Entity>);

/*
#[derive(Default, Resource)]
struct Actions {
	pub player_movement: Option<Vec2>,
}
*/


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
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(spawn_player))
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

/*
fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
	let player_movement = Vec2::new(
		get_movement(Direction::Right, &keyboard_input)
			- get_movement(Direction::Left, &keyboard_input),
		get_movement(Direction::Up, &keyboard_input)
			- get_movement(Direction::Down, &keyboard_input),
	);

	if player_movement != Vec2::ZERO {
		actions.player_movement = Some(player_movement.normalize());
	} else {
		actions.player_movement = None;
	}
}
*/

fn spawn_player(mut commands: Commands, textures: Res<loading::TextureAssets>) {
	commands
		.spawn(SpriteBundle {
			texture: textures.crab.clone(),
			transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
			..Default::default()
		})
		/*
		.insert(Player{
			name: "ninjapiraatti".to_string(),
			head: PlayerHead { direction: Direction::Up, position: Position { x: 10, y: 10 } }
		})
		*/
		.insert(PlayerHead {
			direction: Direction::Up,
		})
		.insert(game::Position {
			x: 100,
			y: 100,
		})
		.insert(Player{name: "ninjapiraatti".to_string()});
}

fn move_players(
	mut heads: Query<(Entity, &PlayerHead)>,
	//mut players: Query<(Entity, &Player)>,
	mut positions: Query<&mut game::Position>,
	mut positions2: Query<&mut Transform, With<Player>>,
) {
	if let Some((head_entity, head)) = heads.iter_mut().next() {
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

	}
}

fn grow_player_tails(
	commands: Commands,
	head_positions: Query<&game::Position, With<PlayerHead>>,
	mut segments: ResMut<PlayerSegments>,
	//materials: Res<loading::Materials>,
) {
	//println!("head_positions: {:?}", head_positions);
	segments.0.push(spawn_segment( // This would add the tail always to the same player
		commands,
		//head_positions.single().unwrap().clone().into(),
		head_positions.single().clone()
	));
}

fn spawn_segment(
	mut commands: Commands,
	position: game::Position,
) -> Entity {
	commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: Color::rgb(0.25, 0.25, 0.75),
				custom_size: Some(Vec2::new(5.0, 5.0)),
				..default()
			},
        	..default()
		})
		.insert(PlayerSegment)
		.insert(position)
		.id()
}