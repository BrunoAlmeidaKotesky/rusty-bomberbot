use bevy::{
    prelude::{Component, Vec3}, 
    time::{Timer, TimerMode}, 
    sprite::SpriteBundle,
    ecs::bundle::Bundle
};
use bevy_ecs_ldtk::Worldly;
use bevy_ggrs::Rollback;
use crate::checksum::Checksum;
use super::{collidable::{ColliderBundle}, online::RoundEntity};

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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player_sprite: SpriteBundle,
    pub player: Player,
    pub check_sum: Checksum,
    pub rollback: Rollback,
    pub round_entity: RoundEntity,
    //#[worldly]
    pub worldly: Worldly,
    //#[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}