use bevy::{
	prelude::*,
};

use crate::state;
use super::{despawn_screen};

pub struct UiPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for UiPlugin {
	fn build(&self, app: &mut App) {
		// As this plugin is managing the splash screen, it will focus on the state `state::AppState::Splash`
		app
			// When entering the state, spawn everything needed for this screen
			.add_system_set(SystemSet::on_enter(state::AppState::MainMenu).with_system(ui_setup))
			.add_system_set(
				SystemSet::on_exit(state::AppState::MainMenu)
					.with_system(despawn_screen::<OnMainMenu>),
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
) {
	for (interaction, mut color, children) in &mut interaction_query {
		let mut text = text_query.get_mut(children[0]).unwrap();
		match *interaction {
			Interaction::Clicked => {
				text.sections[0].value = "Press".to_string();
				*color = PRESSED_BUTTON.into();
			}
			Interaction::Hovered => {
				text.sections[0].value = "Hover".to_string();
				*color = HOVERED_BUTTON.into();
			}
			Interaction::None => {
				text.sections[0].value = "Button".to_string();
				*color = NORMAL_BUTTON.into();
			}
		}
	}
}

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn(ButtonBundle {
					style: Style {
						size: Size::new(Val::Px(150.0), Val::Px(65.0)),
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