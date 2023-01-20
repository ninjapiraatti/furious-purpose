use bevy::{
	prelude::*,
	ecs::schedule::ReportExecutionOrderAmbiguities,
	winit::WinitSettings,
};
mod player;
mod state;
mod ui;
mod assets;

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
		.add_plugins(DefaultPlugins)
		.add_plugin(player::HelloPlugin)
		.add_system(player::lol)
		.add_system(ui::button_system)
		.run();
}
