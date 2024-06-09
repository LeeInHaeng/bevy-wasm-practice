use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::NUMBER_OF_ENEMIES;
use super::ENEMY_SPEED;
use super::ENEMY_SIZE;
use crate::game::player::PLAYER_SIZE;
use crate::game::player::components::Player;
use crate::game::score::resources::Score;
use crate::game::components::GameOver;
use crate::AppState;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>()*window.width();
        let random_y = random::<f32>()*window.height();

        commands.spawn((
           SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
           },
           Enemy{
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
           },
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = half_enemy_size + 0.1;
    let x_max = window.width() - half_enemy_size - 0.1;
    let y_min = half_enemy_size + 0.1;
    let y_max = window.height() - half_enemy_size - 0.1;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_change = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_change = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_change = true;
        }

        if direction_change {
            let sound_effect_1 = asset_server.load("audio/footstep_snow_001.ogg");
            let sound_effect_2 = asset_server.load("audio/footstep_grass_001.ogg");
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::ONCE,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer (
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>()*window.width();
        let random_y = random::<f32>()*window.height();

        commands.spawn((
            SpriteBundle {
                 transform: Transform::from_xyz(random_x, random_y, 0.0),
                 texture: asset_server.load("sprites/ball_red_large.png"),
                 ..default()
            },
            Enemy{
                 direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
         ));
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = PLAYER_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game Over");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/impactBell_heavy_001.ogg"),
                    settings: PlaybackSettings::ONCE,
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value } );
                commands.insert_resource(NextState(Some(AppState::GameOver)));
            }
        }
    }
}

pub fn despawn_enemies (
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}