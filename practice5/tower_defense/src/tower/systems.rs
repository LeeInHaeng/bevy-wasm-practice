use bevy::{prelude::*, utils::FloatOrd};
use std::time::Duration;

use super::components::*;
use crate::enemy::components::Enemy;
use crate::bullet::components::Bullet;

pub fn spawn_tower (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Tower { shooting_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating), bullet_offset: Vec3::new(0.0, 0.2, 0.5) },
        Name::new("Tower")
    ));
}

pub fn tower_shooting (
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    asset_server: Res<AssetServer>,
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
                            scene: asset_server.load("tower/Bullet.glb#Scene0"),
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