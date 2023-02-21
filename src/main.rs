mod bomb;
mod camera;
mod checksum;
mod components;
mod constants;
mod input;
mod player;
mod resources;
mod menu;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*
};
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::{GGRSPlugin};
use bomb::{
    explosion_animation_system, explosion_to_spawn_system
};
use camera::CameraPlugin;
use checksum::{Checksum, checksum_players_system};
use components::{RoundEntity, GGRSConfig};
use player::{move_players, spawn_players};
use resources::{GameTextures, FontAssets, LocalHandles, Session};
use std::time::Duration;

fn main() {
    let mut app = App::new();

    init_ggrsp_plugin(&mut app);
    app.add_plugin(CameraPlugin)
        .add_state(AppState::AssetLoading)
        .add_startup_system(load_ldtk_levels)
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
            .continue_to_state(AppState::MenuMain)
            .with_collection::<FontAssets>()
            .with_collection::<GameTextures>()
        )
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<LDTKBundle>("MyEntityIdentifier")
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: Duration::from_secs(10),
            debug: false,
            filter: None,
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // main menu
        .add_system_set(SystemSet::on_enter(AppState::MenuMain)
            .with_system(menu::main::setup_ui)
            .with_system(explosion_animation_system)
            .with_system(explosion_to_spawn_system)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuMain)
                //.with_system(menu::main::btn_visuals)
                .with_system(menu::main::btn_listeners),
        )
        .add_system_set(SystemSet::on_exit(AppState::MenuMain).with_system(menu::main::cleanup_ui))
        //online menu
        .add_system_set(
            SystemSet::on_enter(AppState::MenuOnline).with_system(menu::online::setup_ui),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuOnline)
                .with_system(menu::online::update_lobby_id)
                .with_system(menu::online::update_lobby_id_display)
                .with_system(menu::online::update_lobby_btn)
                //.with_system(menu::online::btn_visuals)
                .with_system(menu::online::btn_listeners),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuOnline).with_system(menu::online::cleanup_ui),
        )
        // connect menu
        .add_system_set(
            SystemSet::on_enter(AppState::MenuConnect)
                .with_system(menu::connect::create_matchbox_socket)
                .with_system(menu::connect::setup_ui)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MenuConnect)
                .with_system(menu::connect::update_matchbox_socket)
                //.with_system(menu::connect::btn_visuals)
                .with_system(menu::connect::btn_listeners)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MenuConnect)
                .with_system(menu::connect::cleanup)
                .with_system(menu::connect::cleanup_ui),
        )
        // local round
        .add_system_set(
            SystemSet::on_enter(AppState::RoundLocal)
                .with_system(spawn_players),
        )
        .add_system_set(SystemSet::on_exit(AppState::RoundLocal).with_system(cleanup))
        // online round
        .add_system_set(
            SystemSet::on_enter(AppState::RoundOnline)
                .with_system(spawn_players),
        )
        .add_system_set(SystemSet::on_exit(AppState::RoundOnline).with_system(cleanup))
        .run();
}

fn init_ggrsp_plugin(app: &mut App) {
    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(60)
        .with_input_system(input::handle_input_system)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Checksum>()
        .with_rollback_schedule(Schedule::default()
            .with_stage(
            "ROLLBACK_STAGE",
            SystemStage::parallel()
                    .with_system(move_players)
            )
            .with_stage_after("ROLLBACK_STAGE", "CHECKSUM_STAGE", SystemStage::parallel()
                .with_system(checksum_players_system)
            )
        )
        .build(app);
}

fn load_ldtk_levels(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/Bomberboy.ldtk"),
        ..default()
    });
}

#[derive(Bundle, LdtkEntity)]
pub struct LDTKBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    RoundLocal,
    RoundOnline,
    Win,
}

pub fn cleanup(query: Query<Entity, With<RoundEntity>>, mut commands: Commands) {
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<Session>();
    //commands.remove_resource::<SessionType<P2PSession<GGRSConfig>>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}