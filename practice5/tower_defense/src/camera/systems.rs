use bevy::prelude::*;

pub fn spawn_camera (
    mut commands: Commands
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Camera")
    ));
}

pub fn camera_controls (
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward().xyz();
    forward.y = 0.0;
    forward = forward.normalize();
    
    let left = camera.left();

    let speed = 3.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::KeyW) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::KeyE) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
}