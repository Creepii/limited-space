use bevy::{
    prelude::{AssetServer, Assets, Handle, Image, Res},
    sprite::TextureAtlas,
};

use crate::atlas::TilemapAtlas;

pub struct Tilemap;

impl Tilemap {
    pub fn width(&self) -> usize {
        todo!()
    }
    pub fn height(&self) -> usize {
        todo!()
    }

    fn get_texture_name_of_tile(&self, x: usize, y: usize) -> &str {
        "tilemap/grass_edge.png"
    }
}

pub struct TilemapAtlasResolver<'a, 'b> {
    tilemap: &'b Tilemap,
    asset_server: Res<'a, AssetServer>,
    tilemap_atlas: Res<'a, TilemapAtlas>,
    atlasses: Res<'a, Assets<TextureAtlas>>,
}

impl<'a, 'b> TilemapAtlasResolver<'a, 'b> {
    pub fn new(
        tilemap: &'b Tilemap,
        asset_server: Res<'a, AssetServer>,
        tilemap_atlas: Res<'a, TilemapAtlas>,
        atlasses: Res<'a, Assets<TextureAtlas>>,
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
        let handle: Handle<Image> = self
            .asset_server
            .get_handle(self.tilemap.get_texture_name_of_tile(x, y));
        atlas.get_texture_index(&handle)
    }
}
