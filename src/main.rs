mod bomb;
mod checksum;
mod components;
mod constants;
mod input;
mod player;
mod resources;
mod menu;
mod plugins;

use std::time::Duration;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*
};
use bevy_ecs_ldtk::prelude::*;
use plugins::{connections_plugin, asset_plugin, ggrsp_plugin::init_ggrsp_plugin, camera::CameraPlugin};

fn main() {
    let mut app = App::new();

    init_ggrsp_plugin(&mut app);
    app.add_plugin(CameraPlugin)
        .add_startup_system(load_ldtk_levels)
        .add_plugin(asset_plugin::AssetLoadingPlugin)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<LDTKBundle>("MyEntityIdentifier")
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: Duration::from_secs(10),
            debug: false,
            filter: None,
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(connections_plugin::MainMenuPlugin)
        .add_plugin(connections_plugin::ConnectMenuPlugin)
        .add_plugin(connections_plugin::OnlineMenuPlugin)
        .add_plugin(connections_plugin::LocalMatchPlugin)
        .add_plugin(connections_plugin::OnlineMatchPlugin)
        .run();
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