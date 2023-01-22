use bevy::{
	prelude::*,
	ecs::schedule::ReportExecutionOrderAmbiguities,
	winit::WinitSettings,
};
use bevy_asset_loader::{
	prelude::*,
};
mod player;
mod state;
mod ui;
mod assets;
mod splash;

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
		.insert_resource(WinitSettings::desktop_app())
		.insert_resource(ReportExecutionOrderAmbiguities)
		//.init_resource::<state::InGameState>()
		.add_loading_state(
			LoadingState::new(state::AppState::Loading)
				.continue_to_state(state::AppState::Splash)
				.with_collection::<assets::FontAssets>()
				.with_collection::<assets::ImageAssets>()
		)
		.add_state(state::AppState::Splash)
		.add_startup_system(setup)
		.add_plugins(DefaultPlugins)
		//.add_plugin(player::HelloPlugin)
		.add_plugin(splash::SplashPlugin)
		.add_plugin(ui::UiPlugin)
		//.add_system(player::lol)
		.add_system(ui::button_system)

		.run();
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
