use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

pub fn spawn_basic_scene (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Name::new("Ground")
    ));
}

pub fn get_default_plugin () -> (PluginGroupBuilder, WorldInspectorPlugin) {
    (
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        WorldInspectorPlugin::new()
    )
}