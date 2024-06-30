use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct TowerAssets {
    pub tower_base_scene: Handle<Scene>,
    pub tomato_tower_scene: Handle<Scene>,
    pub tomato_scene: Handle<Scene>,
    pub tomato_bullet: Handle<Scene>
}