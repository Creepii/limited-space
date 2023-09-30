use bevy::prelude::*;

use crate::{
    loading::TilemapAtlas,
    tilemap::{TileSet, Tilemap, TilemapAtlasResolver, Tiles},
    GameStates, MainCamera,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), setup_level);
        app.insert_resource(CurrentCharacter {
            current: Character::Rabbit,
        });
        app.insert_resource(CurrentCameraMode {
            current: CameraMode::AllCharacters,
        });
        app.add_systems(
            Update,
            switch_camera_mode.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(Update, player_movement.run_if(in_state(GameStates::Level)));
        app.add_systems(Update, camera_movement.run_if(in_state(GameStates::Level)));
        app.add_systems(OnExit(GameStates::Level), cleanup_level);
    }
}

fn switch_camera_mode(keys: Res<Input<KeyCode>>, mut camera_mode: ResMut<CurrentCameraMode>) {
    if keys.just_pressed(KeyCode::F) {
        camera_mode.toggle();
    }
}

#[derive(Component, PartialEq, Eq)]
enum Character {
    Turtle,
    Rabbit,
}

#[derive(Resource)]
struct CurrentCharacter {
    current: Character,
}

enum CameraMode {
    AllCharacters,
    CurrentCharacter,
}
impl CurrentCameraMode {
    fn toggle(&mut self) {
        match self.current {
            CameraMode::AllCharacters => self.current = CameraMode::CurrentCharacter,
            CameraMode::CurrentCharacter => self.current = CameraMode::AllCharacters,
        }
    }
}

#[derive(Resource)]
struct CurrentCameraMode {
    current: CameraMode,
}

fn setup_level(
    asset_server: Res<AssetServer>,
    tilemap_atlas: Res<TilemapAtlas>,
    atlasses: Res<Assets<TextureAtlas>>,
    tiles_atlas: Res<Assets<Tiles>>,
    tile_set_atlas: Res<Assets<TileSet>>,
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/turtle.png"),
            ..default()
        },
        Character::Turtle,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/rabbit.png"),
            transform: Transform::from_xyz(0.0, 64.0, 0.0),
            ..default()
        },
        Character::Rabbit,
    ));

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
                    }),
                    ..default()
                });
            }
        }
    }
}

const PLAYER_SPEED: f32 = 256.0;

fn player_movement(
    time: Res<Time>,
    current: Res<CurrentCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &mut Transform)>,
) {
    for (character, mut transform) in &mut query {
        if &current.current == character {
            let direction = Vec2::new(
                if keys.pressed(KeyCode::A) {
                    -1.0
                } else if keys.pressed(KeyCode::D) {
                    1.0
                } else {
                    0.0
                },
                if keys.pressed(KeyCode::W) {
                    1.0
                } else if keys.pressed(KeyCode::S) {
                    -1.0
                } else {
                    0.0
                },
            );
            let movement = direction.normalize_or_zero() * (PLAYER_SPEED * time.delta_seconds());
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}

fn cleanup_level() {}

const CAMERA_SCALE: f32 = 0.4;
const CAMERA_PADDING: f32 = 128.0;

fn camera_movement(
    character: Res<CurrentCharacter>,
    camera_mode: Res<CurrentCameraMode>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<MainCamera>>,
        Query<&Window>,
        Query<(&Character, &Transform)>,
    )>,
) {
    match camera_mode.current {
        CameraMode::AllCharacters => {
            let character_transforms: Vec<Transform> =
                param_set.p2().iter().map(|c| c.1.clone()).collect();
            let ((min_x, max_x), (min_y, max_y)) = character_transforms
                .iter()
                .map(|t| (t.translation.x, t.translation.y))
                .fold(
                    ((f32::MAX, f32::MIN), (f32::MAX, f32::MIN)),
                    |(px, py), (x, y)| ((x.min(px.0), x.max(px.1)), (y.min(py.0), y.max(py.1))),
                );
            let size_x = (min_x - max_x).abs() + CAMERA_PADDING;
            let size_y = (min_y - max_y).abs() + CAMERA_PADDING;
            let (window_width, window_height) = {
                fn get_size(window: &Window) -> (f32, f32) {
                    (window.width(), window.height())
                }
                get_size(param_set.p1().single())
            };
            let scale = (size_x / window_width).max(size_y / window_height);
            let scale = scale.max(CAMERA_SCALE);
            let center = Vec2::new((max_x + min_x) / 2.0, (max_y + min_y) / 2.0);
            for mut camera_transform in param_set.p0().iter_mut() {
                camera_transform.translation.x = center.x;
                camera_transform.translation.y = center.y;
                camera_transform.scale.x = scale;
                camera_transform.scale.y = scale;
            }
        }
        CameraMode::CurrentCharacter => {
            let character_transform = param_set
                .p2()
                .iter()
                .filter(|c| c.0 == &character.current)
                .map(|c| c.1)
                .next()
                .unwrap()
                .clone();
            for mut camera_transform in param_set.p0().iter_mut() {
                camera_transform.translation.x = character_transform.translation.x;
                camera_transform.translation.y = character_transform.translation.y;
                camera_transform.scale.x = CAMERA_SCALE;
                camera_transform.scale.y = CAMERA_SCALE;
            }
        }
    }
}
