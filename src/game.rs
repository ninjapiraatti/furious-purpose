use bevy::prelude::*;

use super::despawn_screen;
use crate::player;
use crate::state;

pub struct GamePlugin;

#[derive(Component, Debug, Clone, Copy)]
pub enum PlayerTag {
  Player1,
  Player2,
  Player3,
  Player4,
}

pub const ARENA_WIDTH: u32 = 640;
pub const ARENA_HEIGHT: u32 = 360;

const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.95, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.95, 0.75, 0.15);

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<state::AppState>()
      .add_systems(OnEnter(state::AppState::Game), (test_ui, static_ui_texts))
      .add_systems(OnExit(state::AppState::Game), despawn_screen::<OnGame>)
      .add_systems(
        Update,
        (test_system, position_translation, score_update_system)
          .run_if(in_state(state::AppState::Game)),
      );
  }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnGame;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

fn position_translation(
  mut windows: Query<&mut Window>,
  mut q: Query<(&Position, &mut Transform)>,
) {
  fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
  }
  let window = windows.single_mut();
  for (pos, mut transform) in q.iter_mut() {
    transform.translation = Vec3::new(
      convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
      convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
      0.0,
    );
  }
}

fn score_update_system(
  scores: ResMut<state::PlayerScores>,
  mut query: Query<(&PlayerTag, &mut Text)>,
) {
  for (tag, mut text) in query.iter_mut() {
    match tag {
      PlayerTag::Player1 => text.sections[0].value = scores.player1.to_string(),
      PlayerTag::Player2 => text.sections[0].value = scores.player2.to_string(),
      PlayerTag::Player3 => text.sections[0].value = scores.player3.to_string(),
      PlayerTag::Player4 => text.sections[0].value = scores.player4.to_string(),
    }
  }
}

pub fn test_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Pressed => {
        text.sections[0].value = "^ - ^".to_string();
        *color = PRESSED_BUTTON.into();
      }
      Interaction::Hovered => {
        text.sections[0].value = "LOLL".to_string();
        *color = HOVERED_BUTTON.into();
        println!("Hover");
      }
      Interaction::None => {
        //text.sections[0].value = "LOLL".to_string();
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

pub fn static_ui_texts(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(
    TextBundle::from_section(
      "Cookie Crab",
      TextStyle {
        font: asset_server.load("OverpassMono-SemiBold.ttf"),
        font_size: 10.0,
        color: player::COLOR_P1,
      },
    )
    .with_style(Style {
      position_type: PositionType::Absolute,
      top: Val::Px(10.0),
      left: Val::Px(15.0),
      ..default()
    }),
  );
  commands.spawn(
    TextBundle::from_section(
      "Sid Starfish",
      TextStyle {
        font: asset_server.load("OverpassMono-SemiBold.ttf"),
        font_size: 10.0,
        color: player::COLOR_P2,
      },
    )
    .with_style(Style {
      position_type: PositionType::Absolute,
      top: Val::Px(10.0),
      right: Val::Px(15.0),
      ..default()
    }),
  );
  commands.spawn(
    TextBundle::from_section(
      "Foo Frog",
      TextStyle {
        font: asset_server.load("OverpassMono-SemiBold.ttf"),
        font_size: 10.0,
        color: player::COLOR_P3,
      },
    )
    .with_style(Style {
      position_type: PositionType::Absolute,
      bottom: Val::Px(10.0),
      left: Val::Px(15.0),
      ..default()
    }),
  );
  commands.spawn(
    TextBundle::from_section(
      "Jabby Jellyfish",
      TextStyle {
        font: asset_server.load("OverpassMono-SemiBold.ttf"),
        font_size: 10.0,
        color: player::COLOR_P4,
      },
    )
    .with_style(Style {
      position_type: PositionType::Absolute,
      bottom: Val::Px(10.0),
      right: Val::Px(15.0),
      ..default()
    }),
  );
}

pub fn test_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn((
      NodeBundle {
        style: Style {
          display: Display::Grid,
          grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
          grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          align_content: AlignContent::SpaceBetween,
          ..default()
        },
        //background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
      },
      OnGame,
    ))
    .with_children(|parent| {
      parent.spawn((
        TextBundle::from_section(
          "1",
          TextStyle {
            font: asset_server.load("OverpassMono-SemiBold.ttf"),
            font_size: 30.0,
            color: player::COLOR_P1,
          },
        )
        .with_style(Style {
          position_type: PositionType::Absolute,
          top: Val::Px(25.0),
          left: Val::Px(15.0),
          ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        PlayerTag::Player1,
      ));
      parent.spawn((
        TextBundle::from_section(
          "2",
          TextStyle {
            font: asset_server.load("OverpassMono-SemiBold.ttf"),
            font_size: 30.0,
            color: player::COLOR_P2,
          },
        )
        .with_style(Style {
          position_type: PositionType::Absolute,
          top: Val::Px(25.0),
          right: Val::Px(15.0),
          ..default()
        }),
        PlayerTag::Player2,
      ));
      parent.spawn((
        TextBundle::from_section(
          "3",
          TextStyle {
            font: asset_server.load("OverpassMono-SemiBold.ttf"),
            font_size: 30.0,
            color: player::COLOR_P3,
          },
        )
        .with_style(Style {
          position_type: PositionType::Absolute,
          bottom: Val::Px(25.0),
          left: Val::Px(15.0),
          ..default()
        }),
        PlayerTag::Player3,
      ));
      parent.spawn((
        TextBundle::from_section(
          "4",
          TextStyle {
            font: asset_server.load("OverpassMono-SemiBold.ttf"),
            font_size: 30.0,
            color: player::COLOR_P4,
          },
        )
        .with_style(Style {
          position_type: PositionType::Absolute,
          bottom: Val::Px(25.0),
          right: Val::Px(15.0),
          ..default()
        }),
        PlayerTag::Player4,
      ));
    });
}

/*
pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn((
      // These are not some mysterious double parentheses but a tuple
      NodeBundle {
        style: Style {
          width: Val::Px(100.0),
          height: Val::Px(100.0),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnGame,
    ))
    .with_children(|parent| {
      parent
        .spawn(ButtonBundle {
          style: Style {
            width: Val::Px(100.0),
            height: Val::Px(50.0),
            // horizontally center child text
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section(
            "In game",
            TextStyle {
              font: asset_server.load("OverpassMono-SemiBold.ttf"),
              font_size: 20.0,
              color: Color::rgb(0.9, 0.9, 0.9),
            },
          ));
        });
    });
}
*/
