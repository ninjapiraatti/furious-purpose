use bevy::{
	prelude::*,
};

use crate::{
	state,
	//init::ImageAssets
};
use super::{despawn_screen};

// Splash screen
pub struct SplashPlugin;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<state::AppState>()
		.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)))
		//.add_system(splash_setup.in_schedule(OnEnter(state::AppState::Splash)))
		.add_systems(OnEnter(state::AppState::Splash), splash_setup)
		//.add_systems(OnEnter(state::AppState::Splash), splash_setup)
		.add_systems(Update, (
			countdown
		).run_if(in_state(state::AppState::Splash)))
		.add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(state::AppState::Splash)));
	}
}

fn splash_setup(
	mut commands: Commands, 
	//image_assets: Res<ImageAssets>
) {
	println!("IN SPLASH SETUP");
	//let icon = image_assets.logo.clone();
	//commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
	// Display the logo
	/* 
	commands
		.spawn((
			NodeBundle {
				style: Style {
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					
					//size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
					width: Val::Px(200.0),
					height: Val::Auto,
					//size: Size::new(Val::Px(200.0), Val::Auto),
					..default()
				},
				image: UiImage::from(icon),
				..default()
			});
		});
		*/
		commands.spawn(SpriteBundle {
			sprite: Sprite {
					color: Color::rgb(0.25, 0.25, 0.75),
					custom_size: Some(Vec2::new(50.0, 100.0)),
					..default()
			},
			transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
			..default()
	});
}

// Tick the timer, and change state when finished
fn countdown(
	mut next_state: ResMut<NextState<state::AppState>>,
	time: Res<Time>,
	mut timer: ResMut<SplashTimer>,
) {
	println!("In countdown");
	if timer.tick(time.delta()).finished() {
		println!("CHANGE SCENE");
		next_state.set(state::AppState::MainMenu);
	}
	println!("{:?}", timer.elapsed_secs());
}