use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let player = (
        /*
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
         */
        SceneBundle {
            scene: asset_server.load("player/Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed{value: 2.5},
        Player{},
        ThirdPersonCameraTarget,
        Name::new("Player"),
    );

    let flashlight = (
        SpotLightBundle::default(),
        Name::new("flashlight"),
    );

    commands.spawn(player)
        .with_children(|parent| {
            parent.spawn(flashlight);
        });
}

pub fn player_movement (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>
) {
    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let camera = match camera_query.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap()
        };

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *camera.forward();
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += *camera.back();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += *camera.left();
        }

        if keyboard_input.pressed(KeyCode::KeyD)  {
            direction += *camera.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.value * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}