use bevy::{prelude::*, window::*, math::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::prelude::*;

const CELL_DIMENSIONS: Vec2 = vec2(30., 30.);
const GRID_DIMENSIONS: Vec2 = vec2(24., 24.);

const START_POS: Vec2 = vec2(5., 5.);

const BG_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SNAKE_BODY_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const FOOD_COLOR: Color = Color::rgb(0.9, 0.2, 0.2);

const TICK_RATE: f32 = 8.0;

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
                Timer::from_seconds(1.0 / TICK_RATE, TimerMode::Repeating)
            )
        )
        .add_systems(Startup, (setup, spawn_food))
        .add_systems(Update, (
            move_snake, control_snake, tick_timer, check_food_collision, extend_snake_system, spawn_food))
        .add_event::<EatFoodEvent>()
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

#[derive(Component)]
struct SnakeFood {
    position: Vec2
}

#[derive(Resource, Deref, DerefMut)]
struct SnakeUpdateTimer(Timer);

#[derive(Bundle)]
struct SnakeBodyPartBundle {
    spritebundle: SpriteBundle,
    snake_body_part: SnakeBodyPart
}

#[derive(Bundle)]
struct SnakeFoodBundle {
    spritebundle: SpriteBundle,
    snake_food: SnakeFood
}

#[derive(Event)]
struct EatFoodEvent;

impl SnakeBodyPartBundle {
    fn new(spawn_pos: Vec2) -> SnakeBodyPartBundle {
        return SnakeBodyPartBundle {
            spritebundle: SpriteBundle {
                transform: Transform {
                    translation: convert_coordinates(spawn_pos).extend(0.0),
                    scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            },
            snake_body_part: SnakeBodyPart,
        }
    }
}

impl SnakeFoodBundle {
    fn new(spawn_pos: Vec2) -> SnakeFoodBundle {
        return SnakeFoodBundle {
            spritebundle: SpriteBundle {
                transform: Transform {
                    translation: convert_coordinates(spawn_pos).extend(0.0),
                    scale: vec3(CELL_DIMENSIONS.x, CELL_DIMENSIONS.y, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            snake_food: SnakeFood {
                position: spawn_pos
            },
        }
    }
}

fn setup(mut commands: Commands) {
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
            positions: vec![START_POS, START_POS - vec2(1.0, 0.0)]
        },
        Name::new("Snake positions")
    )).id();

    let snake_head = commands.spawn((
        SnakeBodyPartBundle::new(START_POS),
        Name::new("Snake head part")
    )).id();

    let snake_tail = commands.spawn((
        SnakeBodyPartBundle::new(START_POS - vec2(1.0, 0.0)),
        Name::new("Snake tail part")
    )).id();

    commands.entity(snake).add_child(snake_head).add_child(snake_tail);
    
    let mut rng = rand::thread_rng();
    
    commands.spawn((
        SnakeFoodBundle::new(vec2(
            (rng.gen::<f32>() * GRID_DIMENSIONS.x).round(), 
            (rng.gen::<f32>() * GRID_DIMENSIONS.y).round()
        )),
        Name::new("food")
    ));
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
    
    if timer.just_finished() {
        for i in (1..snake.positions.len()).rev() {
            snake.positions[i] = snake.positions[i - 1];
        }

        match snake.direction {
            Direction::Left => { snake.positions[0].x -= 1.0; },
            Direction::Right => { snake.positions[0].x += 1.0; },
            Direction::Up => { snake.positions[0].y -= 1.0; },
            Direction::Down => { snake.positions[0].y += 1.0; }
        }

        if snake.positions[0].x >= GRID_DIMENSIONS.x {
            snake.positions[0].x = 0.0;
        } else if snake.positions[0].x < 0.0 {
            snake.positions[0].x = GRID_DIMENSIONS.x - 1.0;
        }
    
        if snake.positions[0].y >= GRID_DIMENSIONS.y {
            snake.positions[0].y = 0.0;
        } else if snake.positions[0].y < 0.0 {
            snake.positions[0].y = GRID_DIMENSIONS.y - 1.0;
        }
    
        for (i, child) in children.iter().enumerate() {
            if let Ok(mut transform) = snake_part_query.get_mut(*child) {
                transform.translation = convert_coordinates(snake.positions[i]).extend(0.0);
            }
        }
    }
}

fn check_food_collision(
    mut commands: Commands,
    snake_query: Query<&Snake>,
    food_query: Query<(Entity, &SnakeFood)>,
    mut ev_eat_food: EventWriter<EatFoodEvent>
) {
    let snake = snake_query.single();
    
    for (entity, food) in &food_query {
        for position in &snake.positions {
            if food.position == *position {
                commands.entity(entity).despawn();
                
                ev_eat_food.send(EatFoodEvent);
            }
        }
    }
}

fn spawn_food(mut commands: Commands, mut ev_eat_food: EventReader<EatFoodEvent>) {
    for _ in ev_eat_food.read() {
        let mut rng = rand::thread_rng();
    
        commands.spawn(SnakeFoodBundle::new(vec2(
            (rng.gen::<f32>() * GRID_DIMENSIONS.x - 1.0).round(), 
            (rng.gen::<f32>() * GRID_DIMENSIONS.y - 1.0).round()
        )));
    }
}

fn extend_snake_system(
    mut commands: Commands, 
    mut snake_query: Query<(Entity, &mut Snake)>, 
    mut ev_eat_food: EventReader<EatFoodEvent>
) {
    for _ in ev_eat_food.read() {
        let (snake_entity, mut snake) = snake_query.single_mut();
        
        let tail_pos = *snake.positions.last().expect("snake tail position");
        
        snake.positions.push(tail_pos);

        let new_snake_part = commands.spawn(
            SnakeBodyPartBundle::new(tail_pos)
        ).id();
        
        let mut snake_entity = commands.entity(snake_entity);
    
        snake_entity.add_child(new_snake_part);

    }
}

fn control_snake(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    let mut snake = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Left) && snake.direction != Direction::Right 
        { snake.direction = Direction::Left; }
    if keyboard_input.just_pressed(KeyCode::Right) && snake.direction != Direction::Left 
        { snake.direction = Direction::Right; }
    if keyboard_input.just_pressed(KeyCode::Up) && snake.direction != Direction::Down 
        { snake.direction = Direction::Up; }
    if keyboard_input.just_pressed(KeyCode::Down) && snake.direction != Direction::Up 
        { snake.direction = Direction::Down; }
}

fn convert_coordinates(grid_coordinates: Vec2) -> Vec2 {
    return vec2(
        (grid_coordinates.x - GRID_DIMENSIONS.x * 0.5) * CELL_DIMENSIONS.x + CELL_DIMENSIONS.x * 0.5, 
        (-grid_coordinates.y + GRID_DIMENSIONS.y * 0.5) * CELL_DIMENSIONS.y - CELL_DIMENSIONS.y * 0.5
    );
}

fn tick_timer(time: Res<Time>, mut timer: ResMut<SnakeUpdateTimer>) {
    timer.tick(time.delta());
}