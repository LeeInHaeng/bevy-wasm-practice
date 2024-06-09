use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{game::components::GameOver, AppState};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>
) {
    for event in game_over_event_reader.read() {
        println!("Your score is : {}", event.score.to_string());
    }
}

pub fn transition_to_game_state (
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        match app_state.get() {
            AppState::Game => {

            },
            _ => {
                commands.insert_resource(NextState(Some(AppState::Game)));
                println!("Entered AppState::Game");
            }
        }
    }
}

pub fn transition_to_main_menu_state (
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match app_state.get() {
            AppState::MainMenu => {

            },
            _ => {
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
                println!("Entered AppState::MainMenu");
            }
        }
    }
}