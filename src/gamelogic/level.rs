use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

use crate::{
    loading::TilemapAtlas,
    tilemap::{TileSet, Tilemap, TilemapAtlasResolver, Tiles},
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
    tiles_atlas: Res<Assets<Tiles>>,
    tile_set_atlas: Res<Assets<TileSet>>,
    mut commands: Commands,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/music.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(1.0)),
            speed: 1.0,
            paused: false,
        },
    });
    let tiles_asset: Handle<Tiles> = asset_server.load("levels/level1/tilemap_ground.csv");
    let tile_set_asset: Handle<TileSet> = asset_server.load("levels/level1/tileset.json");
    let tilemap = Tilemap::new(
        tile_set_atlas.get(&tile_set_asset).unwrap(),
        tiles_atlas.get(&tiles_asset).unwrap(),
    )
    .unwrap();
    let tilemap_resolver =
        TilemapAtlasResolver::new(&tilemap, asset_server, tilemap_atlas, atlasses);
    for x in 0..tilemap.width() {
        for y in 0..tilemap.height() {
            if let Some(tile) = tilemap_resolver.get(x, y) {
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: tilemap_resolver.atlas(),
                    sprite: TextureAtlasSprite::new(tile),
                    transform: Transform::from_translation(Vec3 {
                        x: (x as f32) * 32.0,
                        y: (y as f32) * 32.0,
                        z: 0.0,
                    })
                    .with_scale(Vec3::new(1.02, 1.02, 1.0)),
                    ..default()
                });
            }
        }
    }
}

fn cleanup_level() {}
