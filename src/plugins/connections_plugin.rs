use crate::{
    components::online::{AppState, GGRSConfig, RoundEntity},
    menu::{connect, online, main},
    player::spawn_players,
    resources::{LocalHandles, Session}, bomb::{explosion_animation_system, explosion_to_spawn_system},
};
use bevy::prelude::*;
use bevy_ggrs::Session as SessionType;

pub struct ConnectMenuPlugin;

impl Plugin for ConnectMenuPlugin {
    fn build(&self, app: &mut App) {
        // connect menu
        app.add_system_set(
            SystemSet::on_enter(AppState::MenuConnect)
                .with_system(connect::create_matchbox_socket)
                .with_system(connect::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuConnect)
                .with_system(connect::update_matchbox_socket)
                .with_system(connect::btn_listeners),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuConnect)
                .with_system(connect::cleanup)
                .with_system(connect::cleanup_ui),
        );
    }
}

pub struct OnlineMenuPlugin;

impl Plugin for OnlineMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MenuOnline).with_system(online::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuOnline)
                .with_system(online::update_lobby_id)
                .with_system(online::update_lobby_id_display)
                .with_system(online::update_lobby_btn)
                .with_system(online::btn_listeners),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuOnline).with_system(online::cleanup_ui),
        );
    }
}

pub struct LocalMatchPlugin;

impl Plugin for LocalMatchPlugin {
    fn build(&self, app: &mut App) {
        // local round
        app.add_system_set(SystemSet::on_enter(AppState::RoundLocal).with_system(spawn_players))
            .add_system_set(SystemSet::on_exit(AppState::RoundLocal).with_system(cleanup));
    }
}

pub struct OnlineMatchPlugin;

impl Plugin for OnlineMatchPlugin {
    fn build(&self, app: &mut App) {
        // online round
        app.add_system_set(SystemSet::on_enter(AppState::RoundOnline).with_system(spawn_players))
            .add_system_set(SystemSet::on_exit(AppState::RoundOnline).with_system(cleanup));
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // main menu
        app.add_system_set(SystemSet::on_enter(AppState::MenuMain)
            .with_system(main::setup_ui)
            .with_system(explosion_animation_system)
            .with_system(explosion_to_spawn_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuMain)
                .with_system(main::btn_listeners),
        )
        .add_system_set(SystemSet::on_exit(AppState::MenuMain).with_system(main::cleanup_ui));
    }
}

pub fn cleanup(query: Query<Entity, With<RoundEntity>>, mut commands: Commands) {
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<Session>();
    commands.remove_resource::<SessionType<GGRSConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
