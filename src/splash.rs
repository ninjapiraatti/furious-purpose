use bevy::prelude::*;

use super::despawn_screen;
use crate::{init::ImageAssets, state};

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
    app
      .add_state::<state::AppState>()
      .insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)))
      .add_systems(OnEnter(state::AppState::Splash), splash_setup)
      .add_systems(
        Update,
        (countdown).run_if(in_state(state::AppState::Splash)),
      )
      .add_systems(
        OnExit(state::AppState::Splash),
        despawn_screen::<OnSplashScreen>,
      );
  }
}

fn splash_setup(mut commands: Commands, image_assets: Res<ImageAssets>) {
  let icon = image_assets.logo.clone();
  commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));

  commands
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
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
          ..default()
        },
        image: UiImage::from(icon),
        ..default()
      });
    });
}

// Tick the timer, and change state when finished
fn countdown(
  mut next_state: ResMut<NextState<state::AppState>>,
  time: Res<Time>,
  mut timer: ResMut<SplashTimer>,
) {
  if timer.tick(time.delta()).finished() {
    next_state.set(state::AppState::MainMenu);
  }
}
