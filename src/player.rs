use std::ops::Range;

use crate::checksum::Checksum;
use crate::components::{BombBag, GGRSConfig, Player, RoundEntity};
use crate::constants::MAX_PLAYERS;
use crate::input::control_direction;
use crate::resources::GameTextures;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::PlayerInputs;
use bevy_ggrs::{Rollback, RollbackIdProvider};

fn player_color(index: usize) -> Color {
    match index {
        0 => Color::rgb(1., 0., 0.),
        1 => Color::rgb(0., 1., 0.),
        _ => Color::rgb(0.27, 0.27, 0.27),
    }
}
pub fn spawn_players(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    game_texture: Res<GameTextures>,
) {
    const RANGE: Range<usize> = 0..MAX_PLAYERS;
    for (handle, index) in RANGE.enumerate() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 100.),
                    scale: Vec3::new(0.10, 0.10, 0.10),
                    ..default()
                },
                sprite: Sprite {
                    color: player_color(index),
                    ..default()
                },
                texture: game_texture.player.clone(),
                ..default()
            },
            Player { handle },
            BombBag::default(),
            Checksum::default(),
            Rollback::new(rip.next_id()),
            RoundEntity,
        ));
    }
}

pub fn move_players(
    inputs: Res<PlayerInputs<GGRSConfig>>,
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
) {
    for (mut transform, player) in player_query.iter_mut() {
        let input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0,
            InputStatus::Predicted => inputs[player.handle].0,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        let direction = control_direction(&input);
        if direction == Vec2::ZERO {
            continue;
        }
        let move_speed = 0.13;
        let move_delta = direction * move_speed;
        //it should not move on diagonal directions
        let move_delta = if move_delta.x.abs() > move_delta.y.abs() {
            Vec2::new(move_delta.x, 0.)
        } else {
            Vec2::new(0., move_delta.y)
        };

        let old_pos = transform.translation.xy();
        let limit = Vec2::splat(5000. / 2. - 0.5);
        let new_pos = (old_pos + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}
