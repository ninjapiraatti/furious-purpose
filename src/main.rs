use bevy::prelude::*;
mod assets;

#[derive(Component)]
struct Position { x: f32, y: f32 }

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

struct Entity(u64);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(assets::HelloPlugin)
        .add_system(assets::lol)
        .run();
}
