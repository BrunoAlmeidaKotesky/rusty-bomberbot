use crate::{
    constants::MAX_PLAYERS,
    resources::{FontAssets, LocalHandles, LobbyID},
    components::{online::{AppState, GGRSConfig}},
};
use bevy::prelude::*;
use bevy_ggrs::{
    ggrs::{PlayerType, SessionBuilder},
    Session as SessionType,
};

#[derive(Component)]
pub struct MenuMainUI;

#[derive(Component)]
pub enum MenuMainBtn {
    OnlineMatch,
    LocalMatch
}

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
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
            MenuMainUI,
        ))
        .with_children(|parent| {
            // online match button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                            "Online",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: Color::CYAN,
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuMainBtn::OnlineMatch);

            // local mode button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
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
                            "Local",
                            TextStyle {
                                font: font_assets.default_font.clone(),
                                font_size: 40.0,
                                color: Color::CYAN,
                            },
                        ),
                        ..default()
                    });
                })
                .insert(MenuMainBtn::LocalMatch);
        })
        .insert(MenuMainUI);
}

pub fn btn_listeners(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<(&Interaction, &MenuMainBtn), Changed<Interaction>>,
) {
    for (interaction, btn) in interaction_query.iter_mut() {
        if let Interaction::Clicked = *interaction {
            match btn {
                MenuMainBtn::OnlineMatch => {
                    state
                        .set(AppState::MenuOnline)
                        .expect("Could not change state.");
                }
                MenuMainBtn::LocalMatch => {
                    create_synctest_session(&mut commands);
                    state
                        .set(AppState::RoundLocal)
                        .expect("Could not change state.");
                }
            }
        }
    }
}

pub fn cleanup_ui(query: Query<Entity, With<MenuMainUI>>, mut commands: Commands) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn create_synctest_session(commands: &mut Commands) {
    let mut session_build = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(MAX_PLAYERS)
        .with_max_prediction_window(10)
        .with_fps(60)
        .expect("Invalid FPS")
        .with_input_delay(2)
        .with_check_distance(2);

    for i in 0..2 {
        session_build = session_build
            .add_player(PlayerType::Local, i)
            .expect("Could not add local player");
    }

    let session = session_build.start_synctest_session().expect("");

    commands.insert_resource(SessionType::SyncTestSession(session));
    commands.insert_resource(LocalHandles {
        handles: (0..2).collect(),
        lobby_id: Some(LobbyID("local".to_owned()))
    });
}
