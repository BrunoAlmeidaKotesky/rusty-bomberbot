use crate::constants::{INPUT_DOWN, INPUT_FIRE, INPUT_LEFT, INPUT_RIGHT, INPUT_UP};
use bevy::prelude::*;
use bevy_ggrs::ggrs::PlayerHandle;

pub fn handle_input_system(_: In<PlayerHandle>, keys: Res<Input<KeyCode>>) -> u8 {
    let mut input: u8 = 0;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        input |= INPUT_UP;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        input |= INPUT_DOWN;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }
    if keys.any_pressed([KeyCode::Space, KeyCode::Z]) {
        input |= INPUT_FIRE;
    }

    input
}

pub fn control_direction(input: &u8) -> Vec2 {
    let mut direction = Vec2::ZERO;

    if input & INPUT_UP != 0 {
        direction.y += 1.;
    }
    if input & INPUT_DOWN != 0 {
        direction.y -= 1.;
    }
    if input & INPUT_RIGHT != 0 {
        direction.x += 1.;
    }
    if input & INPUT_LEFT != 0 {
        direction.x -= 1.;
    }

    direction
}
