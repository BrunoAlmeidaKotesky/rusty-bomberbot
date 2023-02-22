use bevy::{
    prelude::{Component, Vec3},
    time::{Timer, TimerMode},
};
use bevy_ggrs::ggrs::{Config};

#[derive(Component)]
pub struct FuseTime {
    /// track when the bomb should explode (non-repeating timer).
    pub timer: Timer,
}

#[derive(Component)]
pub struct Player {
    pub handle: usize,
}
#[derive(Component)]
pub struct Explosion;
#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);
#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct BombSlot {
    pub bomb: Option<Bomb>,
}

#[derive(Component)]
pub struct Bomb;

#[derive(Component)]
pub struct BombBag {
    pub slots: [BombSlot; 9],
}

impl Default for BombBag {
    fn default() -> Self {
        Self {
            slots: [
                BombSlot { bomb: Some(Bomb) },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
                BombSlot { bomb: None },
            ],
        }
    }
}

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
    RoundOnline
}