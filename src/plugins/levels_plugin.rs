use crate::{
    components::{AppState, Player},
    resources::{LDTKBundle, LobbyID, LocalHandles},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashSet;

const LEVEL_IIDS: [&str; 2] = [
    "fddf20c0-9f30-11ed-9d6b-9fce45576d40",
    "0780a240-9f30-11ed-b0b1-4b64fc366061",
];

fn load_ldtk_levels(mut commands: Commands, assets: Res<AssetServer>) {
    info!("Loading ldtk levels");
    const MAP_SIZE: u32 = 41;
    const GRID_WIDTH: f32 = 0.05;
    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 - MAP_SIZE as f32 / 2.,
                10.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                i as f32 - MAP_SIZE as f32 / 2.,
                0.,
                1.,
            )),
            sprite: Sprite {
                color: Color::rgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
            ..default()
        });
    }
    let iids: HashSet<String> = LEVEL_IIDS.into_iter().map(|s| s.to_string()).collect();
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/Bomberboy.ldtk"),
        level_set: LevelSet { iids },
        transform: Transform::from_xyz(1., -1., 111.),
        ..default()
    });
}


pub struct LevelsPlugin;
impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        let level_set = |state: AppState| {
            SystemSet::on_enter(state)
                .with_system(load_ldtk_levels)
        };

        app.add_plugin(LdtkPlugin)
            .insert_resource(LdtkSettings {
                // By default, levels are just spawned at the origin of the world.
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                ..default()
            })
            .add_system_set(level_set(AppState::RoundLocal))
            .add_system_set(level_set(AppState::RoundOnline))
            .add_system(toggle_levels);
    }
}

fn toggle_levels(input: Res<Input<KeyCode>>, mut level_sets: Query<&mut LevelSet>) {
    if input.just_pressed(KeyCode::Space) {
        let mut rng = rand::thread_rng();
        let level_to_toggle = LEVEL_IIDS.choose(&mut rng).unwrap().to_string();

        let mut level_set = level_sets.single_mut();
        if level_set.iids.contains(&level_to_toggle) {
            level_set.iids.remove(&level_to_toggle);
        } else {
            info!("Toggling level {}", level_to_toggle);
            level_set.iids.insert(level_to_toggle);
        }
    }
}
