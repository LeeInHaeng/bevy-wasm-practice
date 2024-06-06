use wasm_bindgen::prelude::*;
use bevy::{
     math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, 
     prelude::*, sprite::MaterialMesh2dBundle
};

#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new()
        .add_plugins({
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Let's Play!".to_string(),
                    resolution: (900., 600.).into(),
                    ..default()
                }),
                ..default()
            })
        })
        .add_systems(
            Startup,
            (
                startup_system,
            ),
        )
        .add_systems(
            Update, 
            (
                check_for_collisions,
                move_paddle.before(check_for_collisions),
                apply_velocity.before(check_for_collisions),
            )
        )
        .run();
}

// paddle definition
#[derive(Component)]
struct Paddle;

const BOTTOM_WALL: f32 = -300.;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

#[derive(Component)]
struct Collider;

// ball definition
#[derive(Component)]
struct Ball;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.0, -0.5);

// wall definition
const WALL_THICKNESS: f32 = 10.0;

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;

const PADDLE_PADDING: f32 = 10.0;


fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // use the default 2d camera
    commands.spawn(Camera2dBundle::default());

    // paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider
    ));

    // ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(1.0)).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));
}

// move paddle
const TIME_STEP: f32 = 1.0 / 60.0;
const PADDLE_SPEED: f32 = 500.0;

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    let left_bound = LEFT_WALL + WALL_THICKNESS/2.0 + PADDLE_SIZE.x/2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS/2.0 - PADDLE_SIZE.x/2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

// ball move
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}


fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate().x;

    for transform in &collider_query {
        let collision = collide_with_side(
            BoundingCircle::new(ball_transform.translation.truncate(), ball_size / 2.),
            Aabb2d::new(transform.translation.truncate(), transform.scale.truncate()/2.),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center);
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}