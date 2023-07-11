use bevy::{
	prelude::*,
};
use bevy_asset_loader::{
	prelude::*,
};

use crate::state;

pub struct InitPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for InitPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_loading_state(
				LoadingState::new(state::AppState::Loading)
					.continue_to_state(state::AppState::Splash)
			)
			.add_collection_to_loading_state::<_, FontAssets>(state::AppState::Loading)
			.add_collection_to_loading_state::<_, ImageAssets>(state::AppState::Loading)
			.add_collection_to_loading_state::<_, TextureAssets>(state::AppState::Loading)
			.add_systems(Update, log_loading_done.run_if(in_state(state::AppState::Splash)));
	}
}

#[derive(Resource)]
pub struct Materials {
	pub head_material: Handle<ColorMaterial>,
	pub segment_material: Handle<ColorMaterial>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
	#[asset(path = "OverpassMono-SemiBold.ttf")]
	pub overpass: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
	#[asset(path = "logo.png")]
	pub logo: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub struct TextureAssets {
	#[asset(path = "crab.png")]
	pub crab: Handle<Image>,
	#[asset(path = "starfish.png")]
	pub starfish: Handle<Image>,
	#[asset(path = "frog.png")]
	pub frog: Handle<Image>,
	#[asset(path = "jellyfish.png")]
	pub jellyfish: Handle<Image>,
}

fn log_loading_done() {
	println!("Loading done");
}

/*
fn use_asset_handles(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.background.clone());
}
*/

