use bevy::prelude::*;

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(15.0, 15.0)),
            material: materials.add(Color::DARK_GREEN),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}