use bevy::prelude::*;

use super::components::*;

pub fn spawn_enemy (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.4, 0.4, 0.4)),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92)),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        },
        Enemy { speed: 0.3, health: 3 },
        Name::new("Enemy")
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.4, 0.4, 0.4)),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92)),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..default()
        },
        Enemy { speed: 0.3, health: 3 },
        Name::new("Enemy")
    ));
}

pub fn move_enemy (
    mut enemies: Query<(&Enemy, &mut Transform)>,
    time: Res<Time>
) {
    for (enemy, mut transform) in &mut enemies {
        transform.translation.x += enemy.speed * time.delta_seconds();
    }
}

pub fn enemy_death (
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy)>
) {
    for (entity, enemy) in &enemies {
        if enemy.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}