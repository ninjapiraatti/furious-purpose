use bevy::{
  prelude::*,
  prelude::{Input, KeyCode, Res},
};

use crate::{game, init, state};
use rand::Rng;
use std::collections::HashMap;

pub struct PlayerPlugin;

#[derive(Component, Debug, Clone)]
struct Player {
  name: String,
  player_tag: game::PlayerTag,
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

#[derive(Resource, Default)]
pub struct PlayerSegments(pub HashMap<String, Vec<Entity>>);

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `state::AppState::Game`
impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_state::<state::AppState>().add_systems(
      Update,
      (
        player_spawn_input,
        player_movement_input,
        move_players,
        grow_player_tails,
      )
        .run_if(in_state(state::AppState::Game)),
    );
  }
}

fn player_spawn_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut in_game_state: ResMut<state::InGameState>,
  mut commands: Commands,
  textures: Res<init::TextureAssets>,
) {
  let mut rng = rand::thread_rng();
  if !in_game_state.player1 && keyboard_input.any_just_pressed([KeyCode::Q, KeyCode::W]) {
    println!("Spawn player 1");
    let start_position = game::Position {
      x: rng.gen_range(100..1100),
      y: rng.gen_range(100..700),
    };
    spawn_player(
      &mut commands,
      &textures,
      "Cookie Crab",
      start_position,
      Direction::Down,
    );
    in_game_state.player1 = true;
  }
  if !in_game_state.player2 && keyboard_input.any_just_pressed([KeyCode::B, KeyCode::N]) {
    println!("Spawn player 2");
    let start_position = game::Position {
      x: rng.gen_range(100..1100),
      y: rng.gen_range(100..700),
    };
    spawn_player(
      &mut commands,
      &textures,
      "Sid Starfish",
      start_position,
      Direction::Down,
    );
    in_game_state.player2 = true;
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

fn spawn_player(
  commands: &mut Commands,
  textures: &Res<init::TextureAssets>,
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
    _ => textures.crab.clone(),
  };
  let tag = match player_name {
    "Cookie Crab" => game::PlayerTag::Player1,
    "Sid Starfish" => game::PlayerTag::Player2,
    "Foo Frog" => game::PlayerTag::Player3,
    "Jabby Jellyfish" => game::PlayerTag::Player4,
    _ => game::PlayerTag::Player1,
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
    .insert(Player {
      name: player_name.to_string(),
      player_tag: tag,
    });
}

fn get_all_positions(
  segments: &PlayerSegments,
  positions: &Query<&mut game::Position>,
  heads: &Query<(Entity, &PlayerHead, &mut Player)>,
) -> Vec<(game::Position, game::PlayerTag)> {
  let mut all_positions = Vec::new();

  // Create a mapping from player name to player tag.
  let player_name_to_tag: HashMap<String, game::PlayerTag> = heads
    .iter()
    .map(|(_, _, player)| (player.name.clone(), player.player_tag))
    .collect();

  for (player_name, segment_entities) in segments.0.iter() {
    let player_tag = player_name_to_tag
      .get(player_name)
      .unwrap_or(&game::PlayerTag::Player1); // Default value or handle error

    for entity in segment_entities {
      if let Ok(pos) = positions.get(*entity) {
        all_positions.push((*pos, *player_tag));
      }
    }
  }

  all_positions
}

fn move_players(
  mut segments: ResMut<PlayerSegments>,
  mut heads: Query<(Entity, &PlayerHead, &mut Player)>,
  mut positions: Query<&mut game::Position>,
  mut in_game_state: ResMut<state::InGameState>,
  mut scores: ResMut<state::PlayerScores>,
  mut commands: Commands,
) {
  let segment_positions = get_all_positions(&segments, &positions, &heads);
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
    {
      game_over_players.push(player.name.clone());
      continue;
    }
    // Assuming all_segment_data is a Vec<(game::Position, game::PlayerTag)>
    for (pos, player_tag) in &segment_positions {
      if *pos == *head_pos {
        println!(
          "Collision detected with segment owned by {:?} at position {:?}",
          player_tag, pos
        );
        game_over_players.push(player.name.clone());
        match player_tag {
          game::PlayerTag::Player1 => scores.player1 += 1,
          game::PlayerTag::Player2 => scores.player2 += 1,
          game::PlayerTag::Player3 => scores.player3 += 1,
          game::PlayerTag::Player4 => scores.player4 += 1,
        }
        continue; // or continue depending on your game logic
      }
    }
  }

  for player_name in &game_over_players {
    if let Some(player_segments) = segments.0.remove(player_name) {
      for segment in player_segments {
        commands.entity(segment).despawn();
      }
    }

    if let Some(head_entity) = heads.iter().find_map(|(entity, _, player)| {
      if player.name == *player_name {
        Some(entity)
      } else {
        None
      }
    }) {
      match player_name.as_str() {
        "Cookie Crab" => in_game_state.player1 = false,
        "Sid Starfish" => in_game_state.player2 = false,
        "Foo Frog" => in_game_state.player3 = false,
        "Jabby Jellyfish" => in_game_state.player4 = false,
        _ => (),
      }
      commands.entity(head_entity).despawn();
    }
  }
}

fn grow_player_tails(
  mut commands: Commands,
  head_positions: Query<(&game::Position, &Player), With<PlayerHead>>,
  mut segments: ResMut<PlayerSegments>,
  game_state: Res<state::InGameState>,
) {
  for (head_position, player) in head_positions.iter() {
    let is_alive = match player.player_tag {
      game::PlayerTag::Player1 => game_state.player1,
      game::PlayerTag::Player2 => game_state.player2,
      game::PlayerTag::Player3 => game_state.player3,
      game::PlayerTag::Player4 => game_state.player4,
    };
    if !is_alive {
      continue;
    }
    let player_segments = segments
      .0
      .entry(player.name.clone())
      .or_insert_with(Vec::new);
    player_segments.push(spawn_segment(
      &mut commands,
      head_position.clone(),
      player.clone(),
    ));
  }
}

fn spawn_segment(commands: &mut Commands, position: game::Position, player: Player) -> Entity {
  let color = match &player.name as &str {
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
