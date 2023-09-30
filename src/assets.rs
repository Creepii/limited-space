use bevy::{
    asset::{AssetLoader, LoadedAsset},
    utils::BoxedFuture,
};

use crate::tilemap::{TileSet, Tiles};

#[derive(Debug)]
enum AssetLoaderError {
    TileSetError,
    TilesError
}

impl std::error::Error for AssetLoaderError {}

impl std::fmt::Display for AssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetLoaderError::TileSetError => write!(f, "Failed to load tileset."),
            AssetLoaderError::TilesError => write!(f, "Failed to load tiles."),
        }
    }
}

#[derive(Default)]
pub struct TileSetAssetLoader;

impl AssetLoader for TileSetAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let tile_set_asset =
                TileSet::from_reader(bytes).ok_or(bevy::asset::Error::new(AssetLoaderError::TileSetError))?;
            load_context.set_default_asset(LoadedAsset::new(tile_set_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Default)]
pub struct TilesAssetLoader;

impl AssetLoader for TilesAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let tiles_asset =
                Tiles::from_reader(bytes).ok_or(bevy::asset::Error::new(AssetLoaderError::TilesError))?;
            load_context.set_default_asset(LoadedAsset::new(tiles_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
