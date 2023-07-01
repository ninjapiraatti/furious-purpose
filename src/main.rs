use bevy::{
	prelude::*,
	ecs::schedule::ReportExecutionOrderAmbiguities,
	window::{PresentMode},
};
use bevy_asset_loader::{
	prelude::*,
};
mod player;
mod state;
mod mainmenu;
mod init;
mod splash;
mod game;

fn print_position_system(query: Query<&Transform>) {
	for transform in query.iter() {
		println!("position: {:?}", transform.translation);
	}
}

struct GameRules {
	winning_score: usize,
	max_rounds: usize,
	max_players: usize,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// ui camera
	commands.spawn(Camera2dBundle::default());
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Aninmals".to_string(),
				width: 1200.,
				height: 800.,
				present_mode: PresentMode::AutoVsync,
				..default()
			},
			..default()
		}))
		.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
		.insert_resource(state::InGameState::default())
		.insert_resource(player::PlayerSegments::default())
		.add_system(toggle_vsync)
		.insert_resource(ReportExecutionOrderAmbiguities)
		.add_state(state::AppState::Loading)
		.add_plugin(init::InitPlugin)
		.add_startup_system(setup)
		.add_plugin(splash::SplashPlugin)
		.add_plugin(mainmenu::MainMenuPlugin)
		.add_plugin(game::GamePlugin)
		.add_plugin(player::PlayerPlugin)
		.run();
}

/// This system toggles the vsync mode when pressing the button V.
/// You'll see fps increase displayed in the console.
fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::V) {
        let window = windows.primary_mut();

        window.set_present_mode(if matches!(window.present_mode(), PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        });
        info!("PRESENT_MODE: {:?}", window.present_mode());
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
		println!("Entity: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
