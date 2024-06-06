use wasm_bindgen::prelude::*;
use bevy::{input::keyboard::{self, KeyboardInput}, prelude::*, sprite::MaterialMesh2dBundle};

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
                move_paddle,
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

// ball definition
#[derive(Component)]
struct Ball;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);


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
        Paddle
    ));

    // ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(1.0)).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball
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

    paddle_transform.translation.x = new_paddle_position;
}