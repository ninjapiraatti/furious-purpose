use bevy::prelude::*;

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

#[derive(Resource, Default)]
pub struct PlayerScores {
  pub player1: u32,
  pub player2: u32,
  pub player3: u32,
  pub player4: u32,
}
