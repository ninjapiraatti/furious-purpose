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
		app.add_state::<state::AppState>()
			//.add_system(game_setup.in_schedule(OnEnter(state::AppState::Game)))
			.add_systems(OnEnter(state::AppState::Game), game_setup)
			.add_systems(OnExit(state::AppState::Game), despawn_screen::<OnGame>)
			//.add_system(despawn_screen::<OnGame>.in_schedule(OnExit(state::AppState::Game)))
			.add_systems(Update, (
				test_system,
				player_scores,
				position_translation
			).run_if(in_state(state::AppState::Game)));
			// .add_system(test_system.in_set(OnUpdate(state::AppState::Game)))
			// .add_system(player_scores.in_set(OnUpdate(state::AppState::Game)))
			// .add_system(position_translation.in_set(OnUpdate(state::AppState::Game)));
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

fn position_translation(mut windows: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let mut window = windows.single_mut();
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
	mut text_query: Query<&mut Text>
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
				text.sections[0].value = "LOLL".to_string();
				*color = NORMAL_BUTTON.into();
			}
		}
	}
}

pub fn player_scores(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		// Create a TextBundle that has a Text with a single section.
		TextBundle::from_section(
				// Accepts a `String` or any type that converts into a `String`, such as `&str`
				"hello\nbevy!",
				TextStyle {
						font: asset_server.load("OverpassMono-SemiBold.ttf"),
						font_size: 100.0,
						color: Color::WHITE,
				},
		) // Set the alignment of the Text
		.with_text_alignment(TextAlignment::Center)
		// Set the style of the TextBundle itself.
		.with_style(Style {
				position_type: PositionType::Absolute,
				//bottom: Val::Px(5.0),
				//right: Val::Px(15.0),
				..default()
		}),
		//ColorText,
	));
}

pub fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(( // These are not some mysterious double parentheses but a tuple
			NodeBundle {
				style: Style {
					width: Val::Px(100.0),
					height: Val::Px(100.0),
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