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

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
    let blue_cube = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(4.0, 4.0, 4.0)),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(-5.0, 2.0, 5.0),
            ..default()
        },
        Name::new("Blue Cube")
    );

    let red_cube = (
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(6.0, 1.0, -6.0),
            ..default()
        },
        Name::new("Red Cube")
    );

    commands.spawn(blue_cube);
    commands.spawn(red_cube);
     */

    let mut create_cube = |size: f32, color: Color, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
        (
            PbrBundle {
                mesh: meshes.add(Cuboid::new(size, size, size)),
                material: materials.add(color),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..default()
            },
            Name::new(name)
        )
    };

    commands.spawn(create_cube(4.0, Color::BLUE, (-5.0, 2.0, 5.0), "Blue Cube".to_string()));
    commands.spawn(create_cube(2.0, Color::RED, (6.0, 1.0, -6.0), "Red Cube".to_string()));
}