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
use plugins::{
    connections_plugin, 
    asset_plugin, 
    ggrsp_plugin::init_ggrsp_plugin, 
    camera_plugin::CameraPlugin,
    levels_plugin::LevelsPlugin
};

fn main() {
    let mut app = App::new();

    init_ggrsp_plugin(&mut app);
    app.add_plugin(CameraPlugin)
        .add_plugin(asset_plugin::AssetLoadingPlugin)
        .add_plugin(LogDiagnosticsPlugin {
            wait_duration: Duration::from_secs(4 * 60),
            debug: false,
            filter: None,
        })
        .add_plugin(LevelsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(connections_plugin::MainMenuPlugin)
        .add_plugin(connections_plugin::ConnectMenuPlugin)
        .add_plugin(connections_plugin::OnlineMenuPlugin)
        .add_plugin(connections_plugin::LocalMatchPlugin)
        .add_plugin(connections_plugin::OnlineMatchPlugin)
        .run();
}