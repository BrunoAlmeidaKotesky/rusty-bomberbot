use crate::{
    resources::{ConnectData, FontAssets, LobbyID},
    components::{online::AppState},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct MenuOnlineUI;

#[derive(Component, Debug)]
pub enum MenuOnlineBtn {
    LobbyMatch,
    QuickMatch,
    Back,
}

#[derive(Component)]
pub struct ButtonEnabled(bool);

#[derive(Component)]
pub struct LobbyCodeText;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    // lobby id resource
    commands.insert_resource(LobbyID("".to_owned()));
    // root node
    commands.spawn((
        MenuOnlineUI,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::all(Val::Px(0.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }
    )).with_children(|parent| {
            // lobby id text
            parent.spawn((
                TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Enter a 4-digit ID!\n".to_owned(),
                                style: TextStyle {
                                    font: font_assets.default_font.clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                            },
                            TextSection {
                                value: "".to_owned(),
                                style: TextStyle {
                                    font: font_assets.default_font.clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                },
                LobbyCodeText,
            ));

            // lobby match button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(16.)),
                        padding: UiRect::all(Val::Px(16.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Lobby Match",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuOnlineBtn::LobbyMatch)
                .insert(ButtonEnabled(false));

            // quick match button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(16.)),
                        padding: UiRect::all(Val::Px(16.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Quick Match",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuOnlineBtn::QuickMatch);

            // back button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(16.)),
                        padding: UiRect::all(Val::Px(16.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Back to Menu",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuOnlineBtn::Back);
        })
        .insert(MenuOnlineUI);
}

pub fn update_lobby_id(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut lobby_id: ResMut<LobbyID>
) {
    let lid = &mut lobby_id.0;
    for ev in char_evr.iter() {
        if lid.len() < 4 && ev.char.is_ascii_digit() {
            lid.push(ev.char);
        }
    }
    if keys.just_pressed(KeyCode::Back) {
        let mut chars = lid.chars();
        chars.next_back();
        *lid = chars.as_str().to_owned();
    }
}

pub fn update_lobby_id_display(
    mut query: Query<&mut Text, With<LobbyCodeText>>,
    lobby_id: ResMut<LobbyID>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = lobby_id.0.clone();
    }
}

pub fn update_lobby_btn(
    text_query: Query<&Text, With<LobbyCodeText>>,
    mut btn_query: Query<&mut ButtonEnabled, With<MenuOnlineBtn>>,
) {
    let mut lobby_id_complete = false;
    for text in text_query.iter() {
        if text.sections[1].value.len() == 4 {
            lobby_id_complete = true;
            break;
        }
    }

    for mut enabled in btn_query.iter_mut() {
        enabled.0 = lobby_id_complete;
    }
}

pub fn btn_listeners(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    lobby_id: Res<LobbyID>,
    mut interaction_query: Query<
        (&Interaction, &MenuOnlineBtn, Option<&ButtonEnabled>),
        Changed<Interaction>,
    >,
) {
    for (interaction, btn, enabled) in interaction_query.iter_mut() {
        let clickable = match enabled {
            Some(e) => e.0,
            None => true,
        };

        if !clickable {
            continue;
        }

        if let Interaction::Clicked = *interaction {
            match btn {
                MenuOnlineBtn::LobbyMatch => {
                    commands.insert_resource(ConnectData {
                        lobby_id: format!("bevy{}", lobby_id.0),
                    });
                    state
                        .set(AppState::MenuConnect)
                        .expect("Could not change state.");
                }
                MenuOnlineBtn::QuickMatch => {
                    commands.insert_resource(ConnectData {
                        lobby_id: "bevy?next=2".to_owned(),
                    });
                    state
                        .set(AppState::MenuConnect)
                        .expect("Could not change state.");
                }
                MenuOnlineBtn::Back => {
                    state
                        .set(AppState::MenuMain)
                        .expect("Could not change state.");
                }
            }
        }
    }
}

pub fn cleanup_ui(query: Query<Entity, With<MenuOnlineUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
