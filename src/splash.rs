use bevy::{
	prelude::*,
};

use crate::{state, init::ImageAssets};
use super::{despawn_screen};

// Splash screen
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		// As this plugin is managing the splash screen, it will focus on the state `state::AppState::Splash`
		app
			// When entering the state, spawn everything needed for this screen
			.add_system_set(SystemSet::on_enter(state::AppState::Splash).with_system(splash_setup))
			// While in this state, run the `countdown` system
			.add_system_set(SystemSet::on_update(state::AppState::Splash).with_system(countdown))
			// When exiting the state, despawn everything that was spawned for this screen
			.add_system_set(
				SystemSet::on_exit(state::AppState::Splash)
					.with_system(despawn_screen::<OnSplashScreen>),
			);
	}
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
	let icon = image_assets.logo.clone();
	// Display the logo
	commands
		.spawn((
			NodeBundle {
				style: Style {
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
					..default()
				},
				..default()
			},
			OnSplashScreen,
		))
		.with_children(|parent| {
			parent.spawn(ImageBundle {
				style: Style {
					// This will set the logo to be 200px wide, and auto adjust its height
					size: Size::new(Val::Px(200.0), Val::Auto),
					..default()
				},
				image: UiImage::from(icon),
				..default()
			});
		});
	// Insert the timer as a resource
	commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
	mut game_state: ResMut<State<state::AppState>>,
	time: Res<Time>,
	mut timer: ResMut<SplashTimer>,
) {
	if timer.tick(time.delta()).finished() {
		println!("CHANGE SCENE");
		game_state.set(state::AppState::MainMenu).unwrap();
	}
	println!("{:?}", timer.elapsed_secs());
}