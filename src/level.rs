use bevy::prelude::*;

use crate::{
    atlas::TilemapAtlas,
    tilemap::{Tilemap, TilemapAtlasResolver},
    GameStates,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), setup_level);
        app.add_systems(OnExit(GameStates::Level), cleanup_level);
    }
}

fn setup_level(
    asset_server: Res<AssetServer>,
    tilemap_atlas: Res<TilemapAtlas>,
    atlasses: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let tilemap = TilemapAtlasResolver::new(&Tilemap {}, asset_server, tilemap_atlas, atlasses);
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tilemap.atlas(),
        sprite: TextureAtlasSprite::new(tilemap.get(0, 0).unwrap()),
        transform: Transform::from_translation(Vec3 {
            x: 32.0,
            y: 0.0,
            z: 0.0,
        }),
        ..default()
    });
}

fn cleanup_level() {}
