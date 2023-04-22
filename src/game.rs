use bevy::{
	prelude::*,
};

use crate::state;
use super::{despawn_screen};

pub struct GamePlugin;

pub const ARENA_WIDTH: u32 = 1200;
pub const ARENA_HEIGHT: u32 = 800;

const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.95, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.95, 0.75, 0.15);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			// When entering the state, spawn everything needed for this screen
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(game_setup))
			.add_system_set(SystemSet::on_exit(state::AppState::Game).with_system(despawn_screen::<OnGame>))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(test_system))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(position_translation));
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

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

pub fn test_system(
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	mut text_query: Query<&mut Text>,
	mut game_state: ResMut<State<state::AppState>>,
) {
	for (interaction, mut color, children) in &mut interaction_query {
		let mut text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				text.sections[0].value = "^ - ^".to_string();
				*color = PRESSED_BUTTON.into();
				//game_state.set(state::AppState::Splash).unwrap();
			}
			Interaction::Hovered => {
				text.sections[0].value = "LOLL".to_string();
				*color = HOVERED_BUTTON.into();
				println!("Hover");
			}
			Interaction::None => {
				text.sections[0].value = "LOLL".to_string();
				*color = NORMAL_BUTTON.into();
			}
		}
	}
}

pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(( // These are not some mysterious double parentheses but a tuple
			NodeBundle {
				style: Style {
					size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					..default()
				},
				..default()
			}, OnGame
		))
		.with_children(|parent| {
			parent
				.spawn(
					ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(100.0), Val::Px(50.0)),
						// horizontally center child text
						position_type: PositionType::Absolute,
						position: UiRect { left: Val::Px(10.0), top: Val::Px(10.0), ..default()},
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