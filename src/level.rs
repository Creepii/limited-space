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
        app.insert_resource(CurrentCharacter {
            current: Character::Turtle,
        });
        app.add_systems(Update, player_movement);
        app.add_systems(OnExit(GameStates::Level), cleanup_level);
    }
}

#[derive(Component, PartialEq, Eq)]
enum Character {
    Turtle,
}

#[derive(Resource)]
struct CurrentCharacter {
    current: Character,
}

fn setup_level(
    asset_server: Res<AssetServer>,
    tilemap_atlas: Res<TilemapAtlas>,
    atlasses: Res<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/turtle.png"),
            ..default()
        },
        Character::Turtle,
    ));

    let tilemap = Tilemap {};
    let tilemap_resolver =
        TilemapAtlasResolver::new(&tilemap, asset_server, tilemap_atlas, atlasses);
    for x in 0..tilemap.width() {
        for y in 0..tilemap.height() {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tilemap_resolver.atlas(),
                sprite: TextureAtlasSprite::new(tilemap_resolver.get(x, y).unwrap()),
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

const PLAYER_SPEED: f32 = 64.0;

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
