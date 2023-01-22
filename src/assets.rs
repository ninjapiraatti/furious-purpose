use bevy::{
	prelude::*,
};
use bevy_asset_loader::{
	prelude::*,
};

pub struct Materials {
	head_material: Handle<ColorMaterial>,
	segment_material: Handle<ColorMaterial>,
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