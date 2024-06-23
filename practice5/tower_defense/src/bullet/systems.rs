use bevy::prelude::*;

use super::components::*;
use crate::enemy::components::Enemy;
use crate::tower::components::Lifetime;

pub fn bullet_despawn (
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime), With<Bullet>>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn move_bullets (
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed + time.delta_seconds();
    }
}

pub fn bullet_collision (
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut enemies: Query<(&mut Enemy, &Transform)>,
) {
    for (bullet, bullet_transform) in &bullets {
        for (mut enemy, enemy_transform) in &mut enemies {
            if Vec3::distance(bullet_transform.translation(), enemy_transform.translation) < 0.2 {
                commands.entity(bullet).despawn_recursive();
                enemy.health -= 1;
                break;
            }
        }
    }
}