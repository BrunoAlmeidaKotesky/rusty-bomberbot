use crate::{
    components::{Bomb, BombBag, Explosion, ExplosionTimer, ExplosionToSpawn, FuseTime, Player, GGRSConfig},
    constants::{INPUT_FIRE},
    resources::GameTextures
};
use bevy::prelude::*;
use bevy_ggrs::{ggrs, PlayerInputs};
use std::time::Duration;

pub fn player_place_bomb_system(
    mut commands: Commands,
    inputs: Res<PlayerInputs<GGRSConfig>>,
    game_texture: Res<GameTextures>,
    query: Query<(&Transform, &mut BombBag, &Player), With<Player>>,
) {
    //For each player, check if they pressed the button to place a bomb, and so check on the bomb bag how many they have, so if they have 1, place it
    for (transform, bomb_bag, player) in query.iter() {
        let (input, status) = inputs[player.handle];

        if status != ggrs::InputStatus::Confirmed {
            continue;
        }
        if input & INPUT_FIRE != 0 {
            for slot in bomb_bag.slots.iter() {
                if slot.bomb.is_none() {
                    //Add a new bomb
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: transform.translation,
                                scale: Vec3::new(0.4, 0.4, 1.0),
                                ..default()
                            },
                            texture: game_texture.player_bomb.clone(),
                            ..default()
                        },
                        Bomb,
                        FuseTime {
                            timer: Timer::new(Duration::from_secs(4), TimerMode::Once),
                        },
                    ));
                    //slot.bomb = Some(Bomb);
                }
            }
        }
    }
}

//A function which removes the bomb from the bag, and spawns the explosion sprite
pub fn bomb_explosion_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FuseTime)>,
) {
    for (entity, mut fuse_time) in query.iter_mut() {
        if fuse_time.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            //After despawn, should spawn the explosion sprite
            commands.spawn(ExplosionToSpawn(Vec3::new(50., 50., 0.)));
        }
    }
}

pub fn explosion_to_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (entity, explosion_to_spawn) in query.iter() {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commands.entity(entity).despawn();
    }
}

pub fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut ExplosionTimer), With<Explosion>>,
) {
    for (entity, mut sprite, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= 16 {
                commands.entity(entity).despawn();
            }
        }
    }
}
