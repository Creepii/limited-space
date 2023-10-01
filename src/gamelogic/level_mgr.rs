use std::sync::OnceLock;

use bevy::prelude::*;

use crate::{
    loading::TilemapAtlas,
    tilemap::{TileSet, Tilemap, TilemapAtlasResolver, Tiles},
};

use super::{
    character::{Character, CurrentCharacter, DiscoveredCharacters, PlayerBundle},
    level::create_push_button,
};

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ManagedLevel {
    Level1,
}

static LEVEL_DATAS: OnceLock<Vec<LevelData>> = OnceLock::new();

impl ManagedLevel {
    fn get_data(&self) -> &'static LevelData {
        &LEVEL_DATAS.get_or_init(|| {
            vec![LevelData {
                tileset: "levels/level1/tileset.json".to_string(),
                tilemap_layers: vec!["levels/level1/tilemap_ground.csv".to_string()],
                starting_character: Character::Turtle,
                characters: vec![
                    CharacterData {
                        is_discovered: true,
                        character: Character::Turtle,
                        starting_position: Vec2::new(0.0, 0.0),
                    },
                    CharacterData {
                        is_discovered: false,
                        character: Character::Rabbit,
                        starting_position: Vec2::new(64.0, 0.0),
                    },
                ],
                buttons: vec![ButtonData {
                    position: Vec2::new(64.0, 64.0),
                    index: 0,
                    color: Color::rgb(0.8, 0.2, 0.2),
                }],
            }]
        })[(*self as u8) as usize]
    }
}

struct CharacterData {
    character: Character,
    starting_position: Vec2,
    is_discovered: bool,
}

struct ButtonData {
    index: usize,
    color: Color,
    position: Vec2,
}

struct LevelData {
    tileset: String,
    tilemap_layers: Vec<String>,
    starting_character: Character,
    characters: Vec<CharacterData>,
    buttons: Vec<ButtonData>,
}

#[derive(Component)]
pub struct LevelManager {
    pub current: ManagedLevel,
}

struct LevelLoadContext<'ctx, 'world, 'cmd> {
    data: &'ctx LevelData,
    asset_server: &'ctx Res<'world, AssetServer>,
    tilemap_atlas: &'ctx Res<'world, TilemapAtlas>,
    atlasses: &'ctx Res<'world, Assets<TextureAtlas>>,
    tiles_atlas: &'ctx Res<'world, Assets<Tiles>>,
    tile_set_atlas: &'ctx Res<'world, Assets<TileSet>>,
    commands: &'ctx mut Commands<'world, 'cmd>,
}

impl LevelManager {
    pub fn load_level<'world, 'cmd>(
        &self,
        level: ManagedLevel,
        asset_server: Res<'world, AssetServer>,
        tilemap_atlas: Res<'world, TilemapAtlas>,
        atlasses: Res<'world, Assets<TextureAtlas>>,
        tiles: Res<'world, Assets<Tiles>>,
        tilesets: Res<'world, Assets<TileSet>>,
        mut commands: Commands<'world, 'cmd>,
    ) {
        let data = level.get_data();
        let mut ctx = LevelLoadContext {
            data,
            asset_server: &asset_server,
            tilemap_atlas: &tilemap_atlas,
            atlasses: &atlasses,
            tiles_atlas: &tiles,
            tile_set_atlas: &tilesets,
            commands: &mut commands,
        };
        ctx.create_tilemap();
        ctx.create_buttons();
        ctx.create_characters();
    }
}

impl<'ctx, 'world, 'cmd> LevelLoadContext<'ctx, 'world, 'cmd> {
    fn create_buttons(&mut self) {
        for button in &self.data.buttons {
            create_push_button(
                self.asset_server,
                self.commands,
                Transform::from_xyz(button.position.x, button.position.y, 5.0),
                button.index,
                button.color,
            );
        }
    }

    fn create_tilemap(&mut self) {
        let tiles_asset: Handle<Tiles> = self.asset_server.load(&self.data.tilemap_layers[0]);
        let tile_set_asset: Handle<TileSet> = self.asset_server.load(&self.data.tileset);
        let tilemap = Tilemap::new(
            self.tile_set_atlas.get(&tile_set_asset).unwrap(),
            self.tiles_atlas.get(&tiles_asset).unwrap(),
        )
        .unwrap();
        let tilemap_resolver = TilemapAtlasResolver::new(
            &tilemap,
            self.asset_server,
            self.tilemap_atlas,
            self.atlasses,
        );
        for x in 0..tilemap.width() {
            for y in 0..tilemap.height() {
                if let Some(tile) = tilemap_resolver.get(x, y) {
                    self.commands.spawn(SpriteSheetBundle {
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

    fn create_characters(&mut self) {
        let mut discovered = Vec::new();
        for character in &self.data.characters {
            if character.is_discovered {
                discovered.push(character.character.clone());
            }
            self.commands.spawn((
                SpriteBundle {
                    texture: character.character.texture(self.asset_server),
                    transform: Transform::from_xyz(
                        character.starting_position.x,
                        character.starting_position.y,
                        10.0,
                    ),
                    ..default()
                },
                character.character.clone(),
                character.character.collision_box(),
            ));
        }
        self.commands.spawn(PlayerBundle {
            current: CurrentCharacter {
                current: self.data.starting_character.clone(),
            },
            discovered: DiscoveredCharacters {
                discovered: discovered,
            },
        });
    }
}
