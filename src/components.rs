use std::collections::HashSet;

use bevy::{
    prelude::{Component, Vec3, Entity},
    time::{Timer, TimerMode},
};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_rapier2d::prelude::*;
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

#[derive(Component, Default, Clone, Debug)]
pub struct ColliderWrapper(pub Collider);

impl From<Collider> for ColliderWrapper {
    fn from(collider: Collider) -> ColliderWrapper {
        ColliderWrapper(collider)
    }
}

#[derive(Component)]
pub struct WallSensor {
    pub wall_detection_entity: Entity,
    pub intersecting_wall_entities: HashSet<Entity>,
}

#[derive(Clone, Debug, Default, LdtkIntCell, Component)]
pub struct ColliderBundle {
    pub collider: ColliderWrapper,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: ColliderWrapper(Collider::cuboid(6., 14.)),
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..Default::default()
            },
            "Mob" => ColliderBundle {
                collider: ColliderWrapper(Collider::cuboid(5., 5.)),
                rigid_body: RigidBody::KinematicVelocityBased,
                rotation_constraints,
                ..Default::default()
            },
            "Chest" => ColliderBundle {
                collider: ColliderWrapper(Collider::cuboid(8., 8.)),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity_scale: GravityScale(1.0),
                friction: Friction::new(0.5),
                density: ColliderMassProperties::Density(15.0),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct SensorWrapper(pub Sensor);

impl From<Sensor> for SensorWrapper {
    fn from(sensor: Sensor) -> SensorWrapper {
        SensorWrapper(sensor)
    }
}

#[derive(Component)]
pub struct ActiveEventsWrapper(pub ActiveEvents);

impl From<ActiveEvents> for ActiveEventsWrapper {
    fn from(active_events: ActiveEvents) -> ActiveEventsWrapper {
        ActiveEventsWrapper(active_events)
    }
}

#[derive(Clone, Debug, Default, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: ColliderWrapper,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for SensorBundle {
    fn from(int_grid_cell: IntGridCell) -> SensorBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        // ladder
        if int_grid_cell.value == 2 {
            SensorBundle {
                collider: ColliderWrapper(Collider::cuboid(16., 16.)),
                sensor: Sensor,
                rotation_constraints,
                active_events: ActiveEvents::COLLISION_EVENTS,
            }
        } else {
            SensorBundle::default()
        }
    }
}

#[derive(Clone, Default, Component)]
pub struct WallDetection {
    pub facing_wall: bool,
}

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
