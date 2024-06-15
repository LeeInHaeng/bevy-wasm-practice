use bevy::prelude::*;

pub fn spawn_light(
    mut commands: Commands,
) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 20000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Name::new("MainLight"),
    );

    commands.spawn(light);
}
