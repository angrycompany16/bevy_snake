use bevy::{prelude::*, window::*, math::*, transform};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const CELL_DIMENSIONS: Vec2 = vec2(30., 30.);
const GRID_DIMENSIONS: Vec2 = vec2(24., 24.);

const START_POS: Vec2 = vec2(5., 5.);

const BG_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const FOOD_COLOR: Color = Color::rgb(0.9, 0.2, 0.2);

const TICK_RATE: f32 = 8.;

// TODO: Finish the restructuring

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
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(
            SnakeUpdateTimer(
                Timer::from_seconds(1. / TICK_RATE, TimerMode::Repeating)
            )
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (move_snake, control_snake, tick_timer))
        .run();
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Component)]
struct Snake {
    direction: Direction,
    positions: Vec<Vec2>
}

#[derive(Component)]
struct SnakeBodyPart;

#[derive(Component, Deref, DerefMut)]
struct GridPosition(Vec2);

#[derive(Resource, Deref, DerefMut)]
struct SnakeUpdateTimer(Timer);

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ClearColor(BG_COLOR));

    let snake = commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, 0.0, 0.0),
                scale: vec3(1.0, 1.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
        Snake {
            direction: Direction::Right,
            positions: vec![convert_coordinates(START_POS), convert_coordinates(START_POS - vec2(1., 0.))]
        },
        Name::new("Snake positions")
    )).id();

    let snake_head = commands.spawn((
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
        SnakeBodyPart,
        Name::new("Snake head part")
    )).id();

    let snake_tail = commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: convert_coordinates(START_POS - vec2(1., 0.,)).extend(0.0),
                scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.),
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_COLOR,
                ..default()
            },
            ..default()
        },
        SnakeBodyPart,
        Name::new("Snake head part")
    )).id();
    
    commands.entity(snake).add_child(snake_head).add_child(snake_tail);
}

fn move_snake(
    timer: Res<SnakeUpdateTimer>,
    mut snake_query: Query<(&mut Snake, &Children)>,
    mut snake_part_query: Query<&mut Transform, With<SnakeBodyPart>>
) {
    let (
        mut snake, 
        children
    ) = snake_query.single_mut();
    
    let mut head_pos = snake.positions[0];
    
    if timer.just_finished() {
        match snake.direction {
            Direction::Left => { head_pos.x -= 1.; },
            Direction::Right => { head_pos.x += 1.; },
            Direction::Up => { head_pos.y -= 1.; },
            Direction::Down => { head_pos.y += 1.; }
        }
        
        for i in (snake.positions.len() - 1)..=1 {
            let next_pos = &snake.positions[i - 1];

            snake.positions[i] = *next_pos;
            
            // if grid_position.x < 0. {
            //     grid_position.x = GRID_DIMENSIONS.x;
            // } else if grid_position.x >= GRID_DIMENSIONS.x {
            //     grid_position.x = 0.;
            // }
            
            // if grid_position.y < 0. {
            //     grid_position.y = GRID_DIMENSIONS.y;
            // } else if grid_position.y >= GRID_DIMENSIONS.y {
            //     grid_position.y = 0.;
            // }
        }

        for (i, child) in children.iter().enumerate() {
            if let Ok(mut transform) = snake_part_query.get_mut(*child) {
                transform.translation = snake.positions[i].extend(0.);
            }
        }
    }        

    
    
        // transform.translation = convert_coordinates(grid_position.0).extend(0.0);
}

// fn move_snake_body(
//     commands: Commands,
//     timer: Res<SnakeUpdateTimer>,
//     mut query: Query<(
//         Entity,
//         &mut Transform, 
//         &mut GridPosition
//     ), With<SnakeBodyPart>>
// ) {
    
//     for (entity, transform, grid_position) in &query {
//         // commands.entity(entity).
//     }
// }

fn control_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Snake>
) {
    let mut snake = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Left) {
        if snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        if snake.direction != Direction::Left {
            snake.direction = Direction::Right;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        if snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        if snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }
    }
}

fn convert_coordinates(grid_coordinates: Vec2) -> Vec2 {
    return vec2(
        (grid_coordinates.x - GRID_DIMENSIONS.x * 0.5) * CELL_DIMENSIONS.x + CELL_DIMENSIONS.x * 0.5, 
        (-grid_coordinates.y + GRID_DIMENSIONS.y * 0.5) * CELL_DIMENSIONS.y - CELL_DIMENSIONS.y * 0.5
    );
}

fn tick_timer(
    time: Res<Time>,
    mut timer: ResMut<SnakeUpdateTimer>
) {
    timer.tick(time.delta());
}
























































// use bevy::{prelude::*, window::*, math::*};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

// const CELL_DIMENSIONS: Vec2 = vec2(30., 30.);
// const GRID_DIMENSIONS: Vec2 = vec2(24., 24.);

// const START_POS: Vec2 = vec2(5., 5.);

// const BG_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
// const SNAKE_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// const TICK_RATE: f32 = 8.;

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 resolution: WindowResolution::new(
//                     CELL_DIMENSIONS.x * GRID_DIMENSIONS.x, 
//                     CELL_DIMENSIONS.y * GRID_DIMENSIONS.y
//                 ),
//                 title: "Snake".to_string(),
//                 ..default()
//             }),
//             ..default()
//         }))
//         .add_plugins(WorldInspectorPlugin::new())
//         .insert_resource(
//             SnakeUpdateTimer(
//                 Timer::from_seconds(1. / TICK_RATE, TimerMode::Repeating)
//             )
//         )
//         .add_systems(Startup, setup)
//         .add_systems(Update, (move_snake_body, move_snake_head, control_snake, tick_timer))
//         .run();
// }

// #[derive(PartialEq)]
// enum Direction {
//     Left,
//     Right,
//     Up,
//     Down
// }

// #[derive(Component)]
// struct SnakeHead {
//     direction: Direction,
// }

// #[derive(Component)]
// struct SnakeBodyPart ;

// #[derive(Component, Deref, DerefMut)]
// struct GridPosition(Vec2);

// #[derive(Resource, Deref, DerefMut)]
// struct SnakeUpdateTimer(Timer);

// fn setup(
//     mut commands: Commands
// ) {
//     commands.spawn(Camera2dBundle::default());

//     commands.insert_resource(ClearColor(BG_COLOR));

//     let snake_head = commands.spawn((
//         SpriteBundle {
//             transform: Transform {
//                 translation: convert_coordinates(START_POS).extend(0.0),
//                 scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.),
//                 ..default()
//             },
//             sprite: Sprite {
//                 color: SNAKE_COLOR,
//                 ..default()
//             },
//             ..default()
//         },
//         SnakeHead {
//             direction: Direction::Right
//         },
//         SnakeBodyPart,
//         GridPosition(START_POS),
//         Name::new("Snake head")
//     )).id();

//     let snake_body_part = commands.spawn((
//         SpriteBundle {
//             transform: Transform {
//                 translation: convert_coordinates(START_POS).extend(0.0),
//                 scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.),
//                 ..default()
//             },
//             sprite: Sprite {
//                 color: SNAKE_COLOR,
//                 ..default()
//             },
//             ..default()
//         },
//         SnakeBodyPart,
//         GridPosition(START_POS),
//         Name::new("Snake body part")
//     )).id();

//     // commands.entity(snake_head).add_child(snake_body_part);
// }

// fn tick_timer(
//     time: Res<Time>,
//     mut timer: ResMut<SnakeUpdateTimer>
// ) {
//     timer.tick(time.delta());
// }

// fn move_snake_head(
//     timer: Res<SnakeUpdateTimer>,
//     mut query: Query<(
//         &mut Transform, 
//         &SnakeHead, 
//         &mut GridPosition)>
// ) {
//     let (
//         mut transform, 
//         snake_head, 
//         mut grid_position, 
//     ) = query.single_mut();

//     if timer.just_finished() {
//         match snake_head.direction {
//             Direction::Left => { grid_position.x -= 1.; },
//             Direction::Right => { grid_position.x += 1.; },
//             Direction::Up => { grid_position.y -= 1.; },
//             Direction::Down => { grid_position.y += 1.; }
//         }
    
//         if grid_position.x < 0. {
//             grid_position.x = GRID_DIMENSIONS.x;
//         } else if grid_position.x >= GRID_DIMENSIONS.x {
//             grid_position.x = 0.;
//         }
    
//         if grid_position.y < 0. {
//             grid_position.y = GRID_DIMENSIONS.y;
//         } else if grid_position.y >= GRID_DIMENSIONS.y {
//             grid_position.y = 0.;
//         }
    
//         transform.translation = convert_coordinates(grid_position.0).extend(0.0);
//     }
// }

// fn move_snake_body(
//     commands: Commands,
//     timer: Res<SnakeUpdateTimer>,
//     mut query: Query<(
//         Entity,
//         &mut Transform, 
//         &mut GridPosition
//     ), With<SnakeBodyPart>>
// ) {
    
//     for (entity, transform, grid_position) in &query {
//         // commands.entity(entity).
//     }
// }

// fn convert_coordinates(grid_coordinates: Vec2) -> Vec2 {
//     return vec2(
//         (grid_coordinates.x - GRID_DIMENSIONS.x * 0.5) * CELL_DIMENSIONS.x + CELL_DIMENSIONS.x * 0.5, 
//         (-grid_coordinates.y + GRID_DIMENSIONS.y * 0.5) * CELL_DIMENSIONS.y - CELL_DIMENSIONS.y * 0.5
//     );
// }

// fn control_snake(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<&mut SnakeHead>
// ) {
//     let mut snake_head = query.single_mut();

//     if keyboard_input.just_pressed(KeyCode::Left) {
//         if snake_head.direction != Direction::Right {
//             snake_head.direction = Direction::Left;
//         }
//     }

//     if keyboard_input.just_pressed(KeyCode::Right) {
//         if snake_head.direction != Direction::Left {
//             snake_head.direction = Direction::Right;
//         }
//     }

//     if keyboard_input.just_pressed(KeyCode::Up) {
//         if snake_head.direction != Direction::Down {
//             snake_head.direction = Direction::Up;
//         }
//     }

//     if keyboard_input.just_pressed(KeyCode::Down) {
//         if snake_head.direction != Direction::Up {
//             snake_head.direction = Direction::Down;
//         }
//     }
// }