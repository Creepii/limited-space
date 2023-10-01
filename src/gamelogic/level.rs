use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{
    loading::TilemapAtlas,
    physics::CollisionBox,
    tilemap::{TileSet, Tilemap, TilemapAtlasResolver, Tiles},
    GameStates,
};

#[derive(Component, Debug)]
pub struct PushButton {
    pub pressed: bool,
    pub index: usize,
}

#[derive(Bundle)]
pub struct PushButtonBundle {
    button: PushButton,
    collision: CollisionBox,
    #[bundle()]
    sprite: SpriteBundle,
}

fn play_button_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Ref<PushButton>>,
) {
    for button_ref in &query {
        if button_ref.is_changed() && !button_ref.is_added() {
            commands.spawn(AudioBundle {
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new_absolute(1.0),
                    speed: 1.0,
                    paused: false,
                },
                source: asset_server.load("sounds/button.ogg"),
            });
        }
    }
}

fn create_push_button(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    transform: Transform,
    index: usize,
    color: Color,
) {
    let button_base = asset_server.load("tilemap/push_button_base.png");
    let button = asset_server.load("tilemap/push_button.png");
    commands
        .spawn(PushButtonBundle {
            button: PushButton {
                pressed: false,
                index,
            },
            collision: CollisionBox::Circle { radius: 4.0 },
            sprite: SpriteBundle {
                transform,
                sprite: Sprite { color, ..default() },
                texture: button,
                ..default()
            },
        })
        .with_children(|p| {
            p.spawn(SpriteBundle {
                texture: button_base,
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), setup_level);
        app.add_systems(
            Update,
            play_button_sound.run_if(in_state(GameStates::Level)),
        );
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
            volume: Volume::new_absolute(1.0),
            speed: 1.0,
            paused: false,
        },
    });

    create_push_button(
        &asset_server,
        &mut commands,
        Transform::from_xyz(64.0, 64.0, 5.0),
        0,
        Color::rgb(0.8, 0.2, 0.2),
    );

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
