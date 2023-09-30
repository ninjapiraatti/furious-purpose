use bevy::prelude::*;

use super::despawn_screen;
use crate::state;

pub struct MainMenuPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<state::AppState>()
      .add_systems(OnEnter(state::AppState::MainMenu), ui_setup)
      .add_systems(
        Update,
        (button_system).run_if(in_state(state::AppState::MainMenu)),
      )
      .add_systems(
        OnExit(state::AppState::MainMenu),
        despawn_screen::<OnMainMenu>,
      );
  }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnMainMenu;

pub fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
  mut next_state: ResMut<NextState<state::AppState>>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Pressed => {
        text.sections[0].value = "^ - ^".to_string();
        *color = PRESSED_BUTTON.into();
        next_state.set(state::AppState::Game);
      }
      Interaction::Hovered => {
        text.sections[0].value = "Start".to_string();
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        text.sections[0].value = "Start".to_string();
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn((
      // These are not some mysterious double parentheses but a tuple
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnMainMenu,
    ))
    .with_children(|parent| {
      parent.spawn(
        TextBundle::from_section(
          "Lorem ipsum",
          TextStyle {
            font: asset_server.load("OverpassMono-SemiBold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
          },
        )
        .with_style(Style {
          width: Val::Px(600.0),
          align_items: AlignItems::Center,
          position_type: PositionType::Absolute,
          ..default()
        }),
      );
      parent
        .spawn(ButtonBundle {
          style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            // horizontally center child text
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
            "Button",
            TextStyle {
              font: asset_server.load("OverpassMono-SemiBold.ttf"),
              font_size: 40.0,
              color: Color::rgb(0.9, 0.9, 0.9),
            },
          ));
        });
    });
}
