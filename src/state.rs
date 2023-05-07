use bevy::{
	prelude::*,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
	Loading,
	Splash,
	MainMenu,
	Game,
	Paused,
	GameOver,
}

struct Score {
	value: usize,
}

#[derive(Resource, Default)]
pub struct InGameState {
	current_round: usize,
	total_players: usize,
	winning_player: Option<String>,
	pub player1: bool,
	pub player2: bool,
	pub player3: bool,
	pub player4: bool,
}