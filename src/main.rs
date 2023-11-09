use bevy::{prelude::*, window::*, math::*};

const CELL_DIMENSIONS: Vec2 = vec2(30., 30.);
const GRID_DIMENSIONS: Vec2 = vec2(24., 24.);

const START_POS: Vec2 = vec2(5., 5.);

const BG_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const TICK_RATE: f32 = 8.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    CELL_DIMENSIONS.x * GRID_DIMENSIONS.x, 
                    CELL_DIMENSIONS.y * GRID_DIMENSIONS.y
                ),
                title: "Snake".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake, control_snake))
        .run();
}

enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Component)]
struct Player {
    direction: Direction
}

impl Player {
    fn change_dir(&mut self) {
        self.direction = Direction::Left;
    }
}

trait print {
    
}

#[derive(Component, Deref, DerefMut)]
struct GridPosition(Vec2);

#[derive(Component, Deref, DerefMut)]
struct SnakeUpdateTimer(Timer);

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ClearColor(BG_COLOR));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: convert_coordinates(START_POS).extend(0.0),
                scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.),
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        },
        Player {
            direction: Direction::Right
        },
        GridPosition(START_POS),
        SnakeUpdateTimer(Timer::from_seconds(1. / TICK_RATE, TimerMode::Repeating))
    ));
}

fn move_snake(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform, 
        &Player, 
        &mut GridPosition, 
        &mut SnakeUpdateTimer)>
) {
    let (
        mut transform, 
        player, 
        mut grid_position, 
        mut snake_update_timer
    ) = query.single_mut();

    snake_update_timer.tick(time.delta());
    if snake_update_timer.just_finished() {
        match player.direction {
            Direction::Left => { grid_position.x -= 1.; },
            Direction::Right => { grid_position.x += 1.; },
            Direction::Up => { grid_position.y -= 1.; },
            Direction::Down => { grid_position.y += 1.; }
        }

        if grid_position.x < 0. {
            grid_position.x = GRID_DIMENSIONS.x;
        } else if grid_position.x >= GRID_DIMENSIONS.x {
            grid_position.x = 0.;
        }

        if grid_position.y < 0. {
            grid_position.y = GRID_DIMENSIONS.y;
        } else if grid_position.y >= GRID_DIMENSIONS.y {
            grid_position.y = 0.;
        }
    
        transform.translation = convert_coordinates(grid_position.0).extend(0.0);
    }
}

fn convert_coordinates(grid_coordinates: Vec2) -> Vec2 {
    return vec2(
        (grid_coordinates.x - GRID_DIMENSIONS.x * 0.5) * CELL_DIMENSIONS.x + CELL_DIMENSIONS.x * 0.5, 
        (-grid_coordinates.y + GRID_DIMENSIONS.y * 0.5) * CELL_DIMENSIONS.y - CELL_DIMENSIONS.y * 0.5
    );
}

fn control_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player)>
) {
    let mut player = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Left) {
        match player.direction {
            Direction::Right => {  }
            _ => { player.direction = Direction::Left; }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        match player.direction {
            Direction::Left => {  }
            _ => { player.direction = Direction::Right; }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        match player.direction {
            Direction::Down => {  }
            _ => { player.direction = Direction::Up; }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        match player.direction {
            Direction::Up => {  }
            _ => { player.direction = Direction::Down; }
        }
    }
}