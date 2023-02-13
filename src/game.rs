use bevy::{
	prelude::*,
};

use crate::state;
use super::{despawn_screen};

pub struct GamePlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.65, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.95, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.95, 0.75, 0.15);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			// When entering the state, spawn everything needed for this screen
			.add_system_set(SystemSet::on_enter(state::AppState::Game).with_system(game_setup))
			.add_system_set(SystemSet::on_exit(state::AppState::Game).with_system(despawn_screen::<OnGame>))
			.add_system_set(SystemSet::on_update(state::AppState::Game).with_system(test_system));
	}
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnGame;

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