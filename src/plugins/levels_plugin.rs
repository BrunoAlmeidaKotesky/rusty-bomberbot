use std::collections::HashSet;

use crate::{
    components::{AppState, WallDetection, ColliderWrapper, WallSensor, SensorWrapper, ActiveEventsWrapper}
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

fn load_ldtk_levels(mut commands: Commands, assets: Res<AssetServer>) {
    info!("Loading ldtk levels");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/Bomberboy.ldtk"),
        transform: Transform {
            translation: Vec3::new(0., 0., 11.),
            scale: Vec3::new(0.16, 0.16, 1.),
            ..default()
        },
        ..default()
    });
}


pub fn spawn_ground_sensor(
    mut commands: Commands,
    detect_ground_for: Query<(Entity, &ColliderWrapper), Added<WallDetection>>,
) {
    for (entity, shape) in &detect_ground_for {
        if let Some(cuboid) = shape.0.as_cuboid() {
            let half_extends = cuboid.half_extents();
            let half_extents_x = half_extends.x;
            let half_extents_y = half_extends.y;
            let detector_shape: ColliderWrapper  = Collider::cuboid(half_extents_x / 2.0, 2.).into();

            let sensor_translation = Vec3::new(0., -half_extents_y, 0.);

            commands.entity(entity).with_children(|builder| {
                builder.spawn((
                    ActiveEventsWrapper::from(ActiveEvents::COLLISION_EVENTS),
                    detector_shape,
                    SensorWrapper::from(Sensor),
                    Transform::from_translation(sensor_translation),
                    GlobalTransform::default(),
                    WallSensor {
                        wall_detection_entity: entity,
                        intersecting_wall_entities: HashSet::new()
                    }
                ));
            });
        }
    }
}


pub struct LevelsPlugin;
impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        let level_set = |state: AppState| {
            SystemSet::on_enter(state)
                .with_system(load_ldtk_levels)
        };

        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_system_set(level_set(AppState::RoundLocal))
            .add_system_set(level_set(AppState::RoundOnline));
    }
}