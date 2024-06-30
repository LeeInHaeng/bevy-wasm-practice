use bevy::prelude::*;

use super::components::*;

pub fn enemy_asset_loading (
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(EnemyAssets {
        enemy_scene: assets.load("enemy/Target.glb#Scene0"),
    });
}

pub fn spawn_enemy (
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: enemy_assets.enemy_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        },
        Enemy { speed: 0.3, health: 3 },
        Name::new("Enemy")
    ));

    commands.spawn((
        SceneBundle {
            scene: enemy_assets.enemy_scene.clone(),
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