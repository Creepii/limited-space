use crate::atlas::TilemapAtlas;
use bevy::{
    prelude::{AssetServer, Assets, Handle, Image, Res},
    reflect::{TypePath, TypeUuid},
    sprite::TextureAtlas,
};

#[derive(TypeUuid, TypePath)]
#[uuid = "9ebbbcc1-0fc9-4c4f-841c-21b137bb0173"]
pub struct TileSet {
    texture_names: Vec<String>, // registry names of tiles
}

impl TileSet {
    pub fn from_reader<T: std::io::Read>(reader: T) -> Option<TileSet> {
        let tileset: serde_json::Value = serde_json::from_reader(reader).ok()?;
        let texture_names = tileset["tiles"].as_array().and_then(|tiles| {
            tiles
                .iter()
                .map(|tile| {
                    tile["image"]
                        .as_str()
                        .map(|texture_name: &str| texture_name.to_string())
                })
                .collect()
        })?;
        Some(TileSet {
            texture_names: texture_names,
        })
    }

    fn get_texture_name(&self, tile_type: isize) -> Option<&str> {
        if tile_type < 0 {
            None
        } else {
            self.texture_names
                .get(tile_type as usize)
                .map(|s| s.as_str())
        }
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "4b4a05b4-bfe8-4b1e-9561-e04545132cc3"]
pub struct Tiles {
    tiles: Vec<Vec<isize>>,
}

impl Tiles {
    pub fn from_reader<T: std::io::Read>(mut reader: T) -> Option<Tiles> {
        let mut reader_content = String::new();
        reader.read_to_string(&mut reader_content).ok()?;
        let tiles = reader_content
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|element| element.parse().ok())
                    .collect::<Option<Vec<isize>>>()
            })
            .collect::<Option<Vec<Vec<isize>>>>()?;
        Some(Tiles { tiles: tiles })
    }
}

pub struct Tilemap<'tileset> {
    width: usize,
    height: usize,
    tile_set: &'tileset TileSet,
    tiles: Vec<isize>,
}

impl<'tileset> Tilemap<'tileset> {
    pub fn new(tile_set: &'tileset TileSet, tiles: &Tiles) -> Option<Tilemap<'tileset>> {
        let height = tiles.tiles.len();
        let width = tiles.tiles.iter().map(|row| row.len()).max()?;
        Some(Tilemap {
            width,
            height: height,
            tile_set: tile_set,
            tiles: tiles.tiles.concat(),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn get_texture_name_of_tile(&self, x: usize, y: usize) -> Option<&str> {
        let tile_index = x + self.width() * y;
        self.tiles
            .get(tile_index)
            .and_then(|tile_type| self.tile_set.get_texture_name(*tile_type))
    }
}

pub struct TilemapAtlasResolver<'res, 'tilemap, 'tileset> {
    tilemap: &'tilemap Tilemap<'tileset>,
    asset_server: Res<'res, AssetServer>,
    tilemap_atlas: Res<'res, TilemapAtlas>,
    atlasses: Res<'res, Assets<TextureAtlas>>,
}

impl<'res, 'tilemap, 'tileset> TilemapAtlasResolver<'res, 'tilemap, 'tileset> {
    pub fn new(
        tilemap: &'tilemap Tilemap<'tileset>,
        asset_server: Res<'res, AssetServer>,
        tilemap_atlas: Res<'res, TilemapAtlas>,
        atlasses: Res<'res, Assets<TextureAtlas>>,
    ) -> Self {
        TilemapAtlasResolver {
            tilemap,
            asset_server,
            tilemap_atlas,
            atlasses,
        }
    }

    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.tilemap_atlas.tilemap.as_ref().unwrap().clone()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        let atlas: &TextureAtlas = self.atlasses.get(&self.atlas()).unwrap();
        let texture_name = self.tilemap.get_texture_name_of_tile(x, y)?;
        let handle: Handle<Image> = self.asset_server.get_handle(texture_name);
        atlas.get_texture_index(&handle)
    }
}
