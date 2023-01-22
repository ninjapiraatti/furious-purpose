use bevy::{
	prelude::*,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
	Loading,
	Splash,
	MainMenu,
	InGame,
	Paused,
	GameOver,
}

struct Score {
	value: usize,
}

#[derive(Resource)]
pub struct InGameState {
	current_round: usize,
	total_players: usize,
	winning_player: Option<String>,
}