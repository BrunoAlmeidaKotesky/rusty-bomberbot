use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ggrs::ggrs::PlayerHandle;
use matchbox_socket::WebRtcSocket;

#[derive(Resource)]
pub struct Session {
    pub socket: Option<WebRtcSocket>,
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_bomb: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

#[derive(Resource)]
pub struct ConnectData {
    pub lobby_id: String,
}

#[derive(Resource)]
pub struct LobbyID(pub String);

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(Resource)]
pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
}
