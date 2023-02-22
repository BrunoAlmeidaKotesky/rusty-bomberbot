use crate::{
    checksum::{checksum_players_system, Checksum},
    components::GGRSConfig,
    input,
    player::move_players,
};
use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;

pub fn init_ggrsp_plugin(app: &mut App) {
    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(60)
        .with_input_system(input::handle_input_system)
        .register_rollback_component::<Transform>()
        .register_rollback_component::<Checksum>()
        .with_rollback_schedule(
            Schedule::default()
                .with_stage(
                    "ROLLBACK_STAGE",
                    SystemStage::parallel().with_system(move_players),
                )
                .with_stage_after(
                    "ROLLBACK_STAGE",
                    "CHECKSUM_STAGE",
                    SystemStage::parallel().with_system(checksum_players_system),
                ),
        )
        .build(app);
}
