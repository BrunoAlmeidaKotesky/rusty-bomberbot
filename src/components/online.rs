
use bevy::{
    prelude::{Component},
};
use bevy_ggrs::ggrs::Config;

#[derive(Component)]
pub struct RoundEntity;

//Not exactly considered as components.
pub struct GGRSConfig;
impl Config for GGRSConfig {
    // 4-directions + fire fits easily in a single byte
    type Input = u8;
    type State = u8;
    // Matchbox' WebRtcSocket addresses are strings
    type Address = String;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    RoundLocal,
    RoundOnline,
}
