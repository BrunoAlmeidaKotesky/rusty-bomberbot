use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_ggrs::{ggrs, Session as SessionType};
use ggrs::{PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

use crate::{
    constants::MAX_PLAYERS,
    resources::{ConnectData, FontAssets, LocalHandles, Session},
    components::{AppState, GGRSConfig},
};

#[derive(Component)]
pub struct MenuConnectUI;

#[derive(Component)]
pub enum MenuConnectBtn {
    Back,
}

const MATCHBOX_ADDR: &str = "ws://127.0.0.1:3536";

pub fn create_matchbox_socket(mut commands: Commands, connect_data: Res<ConnectData>) {
    let lobby_id = &connect_data.lobby_id;
    let room_url = format!("{MATCHBOX_ADDR}/{lobby_id}");
    let (socket, message_loop) = WebRtcSocket::new(room_url);
    IoTaskPool::get().spawn(message_loop).detach();
    commands.insert_resource(Session {
        socket: Some(socket),
    });
    commands.remove_resource::<ConnectData>();
}

pub fn update_matchbox_socket(
    commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut session: ResMut<Session>,
) {
    let Some(socket) = &mut session.socket else {
        // If there is no socket we've already started the game
        return;
    };
    socket.accept_new_connections();
    if socket.players().len() >= MAX_PLAYERS {
        // take the socket
        let socket = session.socket.take().unwrap();
        create_ggrs_session(commands, socket);
        state
            .set(AppState::RoundOnline)
            .expect("Could not change state.");
    }
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<Session>();
}

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    // root node
    commands
        .spawn((
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
            },
            MenuConnectUI,
        ))
        .with_children(|parent| {
            // lobby id display
            parent.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                text: Text::from_section(
                    "Searching a match...",
                    TextStyle {
                        font_size: 32.,
                        color: Color::BLACK,
                        font: font_assets.default_font.clone(),
                    },
                ),
                ..default()
            });

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
                .insert(MenuConnectBtn::Back);
        })
        .insert(MenuConnectUI);
}

pub fn btn_listeners(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(&Interaction, &MenuConnectBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuConnectBtn::Back => {
                    state
                        .set(AppState::MenuMain)
                        .expect("Could not change state.");
                }
            }
        }
    }
}

pub fn cleanup_ui(query: Query<Entity, With<MenuConnectUI>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn create_ggrs_session(mut commands: Commands, socket: WebRtcSocket) {
    // create a new ggrs session
    let mut session_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(MAX_PLAYERS)
        .with_max_prediction_window(12)
        .with_fps(60)
        .expect("Invalid FPS")
        .with_input_delay(2);

    // add players
    let mut handles = Vec::new();
    for (i, player_type) in socket.players().iter().enumerate() {
        if *player_type == PlayerType::Local {
            handles.push(i);
        }
        session_build = session_build
            .add_player(player_type.clone(), i)
            .expect("Invalid player added.");
    }

    // start the GGRS session
    let session = session_build
        .start_p2p_session(socket)
        .expect("Session could not be created.");
    //commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession(session));
}
