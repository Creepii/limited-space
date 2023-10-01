use std::sync::OnceLock;

use bevy::prelude::*;

use crate::{
    loading::TilemapAtlas,
    physics::CollisionBox,
    tilemap::{TileSet, Tilemap, TilemapAtlasResolver, Tiles},
};

use super::{
    character::{Character, CurrentCharacter, DiscoveredCharacters, PlayerBundle},
    level::{GoalFlag, GoalFlagBundle, PushButton, PushButtonBundle},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ManagedLevel {
    Level1,
    Level2,
}

static LEVEL_DATAS: OnceLock<Vec<LevelData>> = OnceLock::new();

impl ManagedLevel {
    fn get_data(&self) -> &'static LevelData {
        &LEVEL_DATAS.get_or_init(|| {
            vec![
                LevelData {
                    next_level: Some(ManagedLevel::Level2),
                    flag_position: Vec2::new(128.0, 32.0),
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
                },
                LevelData {
                    next_level: None,
                    flag_position: Vec2::new(256.0, 32.0),
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
                            is_discovered: true,
                            character: Character::Crocodile,
                            starting_position: Vec2::new(64.0, 0.0),
                        },
                    ],
                    buttons: vec![ButtonData {
                        position: Vec2::new(64.0, 64.0),
                        index: 0,
                        color: Color::rgb(0.8, 0.2, 0.2),
                    }],
                },
            ]
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
    next_level: Option<ManagedLevel>,
    tileset: String,
    tilemap_layers: Vec<String>,
    flag_position: Vec2,
    starting_character: Character,
    characters: Vec<CharacterData>,
    buttons: Vec<ButtonData>,
}

#[derive(Component)]
pub struct LevelManager {
    pub current: Option<ManagedLevel>,
    pub next: Option<ManagedLevel>,
}

#[derive(Component)]
pub struct LoadedLevel {
    pub level: ManagedLevel,
}

struct LevelLoadContext<'ctx, 'world, 'cmd> {
    level: ManagedLevel,
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
        asset_server: Res<'world, AssetServer>,
        tilemap_atlas: Res<'world, TilemapAtlas>,
        atlasses: Res<'world, Assets<TextureAtlas>>,
        tiles: Res<'world, Assets<Tiles>>,
        tilesets: Res<'world, Assets<TileSet>>,
        mut commands: Commands<'world, 'cmd>,
    ) {
        info!("Loading level: {:?}", self.next);
        let data = self.next.unwrap().get_data();
        let mut ctx = LevelLoadContext {
            level: self.next.unwrap().clone(),
            data,
            asset_server: &asset_server,
            tilemap_atlas: &tilemap_atlas,
            atlasses: &atlasses,
            tiles_atlas: &tiles,
            tile_set_atlas: &tilesets,
            commands: &mut commands,
        };
        ctx.create_flag();
        ctx.create_tilemap();
        ctx.create_buttons();
        ctx.create_characters();
    }

    pub fn unload_level<'q, I>(&self, commands: &mut Commands, iter: I)
    where
        I: Iterator<Item = (Entity, &'q LoadedLevel)>,
    {
        info!("Unloading level: {:?}", self.current);
        for (entity, loaded) in iter {
            if loaded.level == self.current.unwrap() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

impl<'ctx, 'world, 'cmd> LevelLoadContext<'ctx, 'world, 'cmd> {
    fn create_flag(&mut self) {
        let flag_texture = self.asset_server.load("tilemap/flag.png");
        self.commands.spawn((
            GoalFlagBundle {
                goal_flag: GoalFlag {
                    next_level: self.data.next_level.clone(),
                    reached: false,
                },
                collision: CollisionBox::Circle { radius: 4.0 },
                sprite: SpriteBundle {
                    transform: Transform::from_xyz(
                        self.data.flag_position.x,
                        self.data.flag_position.y,
                        5.0,
                    ),
                    texture: flag_texture,
                    ..default()
                },
            },
            LoadedLevel {
                level: self.level.clone(),
            },
        ));
    }

    fn create_buttons(&mut self) {
        for button_data in &self.data.buttons {
            let button_base = self.asset_server.load("tilemap/push_button_base.png");
            let button = self.asset_server.load("tilemap/push_button.png");
            self.commands
                .spawn((
                    PushButtonBundle {
                        button: PushButton {
                            pressed: false,
                            index: button_data.index,
                        },
                        collision: CollisionBox::Circle { radius: 4.0 },
                        sprite: SpriteBundle {
                            transform: Transform::from_xyz(
                                button_data.position.x,
                                button_data.position.y,
                                5.0,
                            ),
                            sprite: Sprite {
                                color: button_data.color,
                                ..default()
                            },
                            texture: button,
                            ..default()
                        },
                    },
                    LoadedLevel {
                        level: self.level.clone(),
                    },
                ))
                .with_children(|p| {
                    p.spawn(SpriteBundle {
                        texture: button_base,
                        transform: Transform::from_xyz(0.0, 0.0, -1.0),
                        ..default()
                    });
                });
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
                    self.commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: tilemap_resolver.atlas(),
                            sprite: TextureAtlasSprite::new(tile),
                            transform: Transform::from_translation(Vec3 {
                                x: (x as f32) * 32.0,
                                y: (y as f32) * 32.0,
                                z: 0.0,
                            })
                            .with_scale(Vec3::new(1.02, 1.02, 1.0)),
                            ..default()
                        },
                        LoadedLevel {
                            level: self.level.clone(),
                        },
                    ));
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
                LoadedLevel {
                    level: self.level.clone(),
                },
            ));
        }
        self.commands.spawn((
            PlayerBundle {
                current: CurrentCharacter {
                    current: self.data.starting_character.clone(),
                },
                discovered: DiscoveredCharacters {
                    discovered: discovered,
                },
            },
            LoadedLevel {
                level: self.level.clone(),
            },
        ));
    }
}
