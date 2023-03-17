use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingStateAppExt, LoadingState};

use crate::{components::online::AppState, resources::{FontAssets, GameTextures}};

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::AssetLoading).add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::MenuMain)
                .with_collection::<FontAssets>()
                .with_collection::<GameTextures>(),
        );
    }
}
