use bevy::{prelude::*, window::*};

const CELL_DIMENSIONS: Vec2 = Vec2::new(30., 30.);
const GRID_DIMENSIONS: Vec2 = Vec2::new(16., 16.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(CELL_DIMENSIONS.x * GRID_DIMENSIONS.x, CELL_DIMENSIONS.y * GRID_DIMENSIONS.y),
                title: "Snake".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands
) {
    commands.spawn
}