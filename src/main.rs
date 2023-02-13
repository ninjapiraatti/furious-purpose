use bevy::{
	prelude::*,
	ecs::schedule::ReportExecutionOrderAmbiguities,
	winit::WinitSettings,
	window::{CursorGrabMode, PresentMode},
};
use bevy_asset_loader::{
	prelude::*,
};
mod player;
mod state;
mod mainmenu;
mod loading;
mod splash;
mod game;

fn print_position_system(query: Query<&Transform>) {
	for transform in query.iter() {
		println!("position: {:?}", transform.translation);
	}
}

struct Position {
	x: i32,
	y: i32,
}

struct GameRules {
	winning_score: usize,
	max_rounds: usize,
	max_players: usize,
}
/*
fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
	game_state.current_round += 1;
	println!(
		"Begin round {} of {}",
		game_state.current_round, game_rules.max_rounds
	);
}
*/

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	// ui camera
	commands.spawn(Camera2dBundle::default());
}

fn startup_system(
	//mut commands: Commands,
	mut game_state: ResMut<state::InGameState>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Create our game rules resource
	/*
	commands.insert_resource(GameRules {
		max_rounds: 100,
		winning_score: 51,
		max_players: 4,
	});
	commands.spawn_batch(vec![
		(
			Player {
				name: "Quorra".to_string(),
				head: PlayerHead {direction: Direction::Up},
			},
			Score { value: 0 },
		),
		(
			Player {
				name: "Clu".to_string(),
				head: PlayerHead {direction: Direction::Down},
			},
			Score { value: 0 },
		),
	]);
	game_state.total_players = 2;
	*/
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
		.add_system(toggle_vsync)
		//.insert_resource(WinitSettings::desktop_app())
		.insert_resource(ReportExecutionOrderAmbiguities)
		//.init_resource::<state::InGameState>()
		.add_state(state::AppState::Loading)
		.add_plugin(loading::LoadingPlugin)
		.add_startup_system(setup)
		//.add_plugins(DefaultPlugins)
		//.add_plugin(player::HelloPlugin)
		.add_plugin(splash::SplashPlugin)
		.add_plugin(mainmenu::MainMenuPlugin)
		.add_plugin(game::GamePlugin)
		.add_plugin(player::PlayerPlugin)
		//.add_system(player::lol)
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
