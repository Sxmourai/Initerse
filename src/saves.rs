use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub enum Action {
    Up, Down, Left, Right
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct GameConfig {
    window_size: (i32, i32),
    save_title: String,
    fullscreen: bool,

    key_bindings: std::collections::HashMap<String, Action>,

    difficulty_options: DifficultyOptions,
}
#[derive(Debug, Deserialize)]
pub struct DifficultyOptions {
    level: usize
}

#[derive(Default)]
pub struct GameConfigLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum GameConfigLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}


impl AssetLoader for GameConfigLoader {
    type Asset = GameConfig;
    type Settings = ();
    type Error = GameConfigLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = ron::de::from_bytes::<GameConfig>(&bytes)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

// TODO: Move to https://github.com/TheBevyFlock/bevy_new_2d/blob/main/src/asset_tracking.rs
// TODO: Use http://github.com/NiklasEi/bevy_common_assets

