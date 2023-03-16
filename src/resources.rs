use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs_ldtk::prelude::LdtkEntity;
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

#[derive(AssetCollection, Resource)]
pub struct GameTextures {
    #[asset(path = "textures/bomberman_base.png")]
    pub player: Handle<Image>,
    #[asset(path = "textures/bomberman_bomb.png")]
    pub player_bomb: Handle<Image>,
    #[asset(path = "textures/explosion.png")]
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Resource, Debug)]
pub struct ConnectData {
    pub lobby_id: String,
}

#[derive(Resource, Debug, Clone)]
pub struct LobbyID(pub String);

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(Resource, Debug)]
pub struct LocalHandles {
    pub handles: Vec<PlayerHandle>,
    pub lobby_id: Option<LobbyID>
}

#[derive(Resource, Debug)]
pub struct DebugConfig {
    pub enabled: bool,
}

#[derive(Resource, Debug)]
pub struct CameraZoomConfig {
    pub scroll_speed: f32,
    pub min_scale: f32,
    pub max_scale: f32,
}

//Not exactly resources

#[derive(Bundle, LdtkEntity)]
pub struct LDTKBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
