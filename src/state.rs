use bevy::{
	prelude::*,
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
	#[default]
	Loading,
	Splash,
	MainMenu,
	Game,
	Paused,
	GameOver,
}

#[derive(Resource, Default)]
pub struct InGameState {
	//current_round: usize,
	//total_players: usize,
	//winning_player: Option<String>,
	pub player1: bool,
	pub player2: bool,
	pub player3: bool,
	pub player4: bool,
}