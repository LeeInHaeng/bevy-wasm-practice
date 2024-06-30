use bevy::{prelude::*, utils::FloatOrd};
use bevy_mod_picking::PickableBundle;
use std::time::Duration;

use super::components::*;
use crate::enemy::components::Enemy;
use crate::bullet::components::Bullet;

pub fn tower_asset_loading (
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.insert_resource(TowerAssets {
        tower_base_scene: assets.load("tower/TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("tower/TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("tower/Tomato.glb"),
        tomato_bullet: assets.load("tower/Bullet.glb#Scene0"),
    });
}

pub fn spawn_tower (
    mut commands: Commands,
    tower_assets: Res<TowerAssets>,
) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.5, 0.0)),
        Tower { shooting_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating), bullet_offset: Vec3::new(0.0, 0.2, 0.5) },
        Name::new("Tomato_Tower")
    ))
    .with_children(|commands| {
        commands.spawn(
            SceneBundle {
                scene: tower_assets.tomato_tower_scene.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            }
        );
    });
}

pub fn tower_shooting (
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    tower_assets: Res<TowerAssets>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {

            let bullet_translation = transform.translation() + tower.bullet_offset;

            let direction = enemies
                .iter()
                .min_by_key(|enemy_transform| {
                    FloatOrd(Vec3::distance(enemy_transform.translation(), bullet_translation))
                })
                .map(|closest_enemy| {
                    closest_enemy.translation() - bullet_translation
                });

            if let Some(direction) = direction {
                commands.entity(tower_entity).with_children(|commands| {

                    commands.spawn((
                        SceneBundle {
                            scene: tower_assets.tomato_bullet.clone(),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        },
                        Lifetime { timer: Timer::new(Duration::from_secs_f32(10.0), TimerMode::Once)},
                        Bullet { direction: direction, speed: 0.2 },
                        Name::new("Bullet")
                    ));

                });

            }
        }
    }
}