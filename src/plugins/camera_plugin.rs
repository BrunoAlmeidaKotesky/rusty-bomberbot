use bevy::{prelude::*, window::PresentMode};
use crate::{resources::{WinSize, LocalHandles, LobbyID}, components::Player};

pub fn toggle_vsync(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if input.just_pressed(KeyCode::V) {
        let window = windows.primary_mut();

        window.set_present_mode(if matches!(window.present_mode(), PresentMode::AutoVsync) {
            PresentMode::AutoNoVsync
        } else {
            PresentMode::AutoVsync
        });
        info!("PRESENT_MODE: {:?}", window.present_mode());
    }
}

pub fn init_window_plugin() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: "Rust Bomberbot".to_string(),
            fit_canvas_to_parent: true,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }
}

pub fn setup_window_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>
) {
    //Setup camera:
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.02,
            ..default()
        }
        .into(),
        ..default()
    };
    commands.spawn(camera);

	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
}

pub fn camera_follow_system(
    player_handle: Option<Res<LocalHandles>>,
    current_lobby: Option<Res<LobbyID>>,
    player_query: Query<(&Player, &Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {

    let (player_handle, lobby_id) = match player_handle {
        Some(handle) => {
            (handle.handles.first().unwrap().clone(), handle.lobby_id.clone())
        },
        None => {
            return; // Session hasn't started yet
        }
    };

    //This should only work if the player is in the same lobby as the camera

    let current_lobby = match current_lobby {
        Some(ref lobby) => lobby.to_owned(),
        None => {
            return; // Session hasn't started yet
        }
    };

    let lobby_id = match lobby_id {
        Some(ref id) => id,
        None => {
            info!("Camera should not follow player (lobby_id is None)");
            return; // Session hasn't started yet
        }
    };

    for (player, player_transform) in player_query.iter() {
        if player.handle != player_handle && current_lobby.0 != lobby_id.0 {
            //"Camera should not follow player ({}!= {})", player.handle, current_lobby.0);
            continue;
        }
        else if  player.handle != player_handle && current_lobby.0 == lobby_id.0 {
            //"Camera should not follow different player from the same lobby
            continue;
        }

        let player_pos = player_transform.translation;
        for mut transform in camera_query.iter_mut() {
            transform.translation.x = player_pos.x;
            transform.translation.y = player_pos.y;
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            DefaultPlugins
                .set(init_window_plugin())
                .set(ImagePlugin::default_nearest()),
        )
        .add_startup_system(setup_window_system)
        .add_system(camera_follow_system)
        .add_system(toggle_vsync);
    }
}