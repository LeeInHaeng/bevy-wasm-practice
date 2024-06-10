use bevy::prelude::*;
use bevy_third_person_camera::{camera::Zoom, ThirdPersonCamera};

pub fn spawn_camera(
    mut commands: Commands,
) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            //mouse_sensitivity: 5.0,
            zoom: Zoom::new(1.0, 10.0),
            ..default()
        },
    );

    commands.spawn(camera);
}