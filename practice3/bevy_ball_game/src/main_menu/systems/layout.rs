use bevy::prelude::*;

use crate::main_menu::{components::{MainMenu, PlayButton, QuitButton}, styles::*};

pub fn spawn_main_menu (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            ..default()
        },
        MainMenu{},
    ))
    .with_children(|parent| {
        // title
        parent.spawn(
            ButtonBundle {
                style: get_button_style(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Title",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });

        // play button
        parent.spawn((
            ButtonBundle {
                style: get_button_style(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            PlayButton{}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // quit button
        parent.spawn((
            ButtonBundle {
                style: get_button_style(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton{}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    })
    ;
}

pub fn despawn_main_menu (
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}