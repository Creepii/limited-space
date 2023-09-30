use bevy::{asset::HandleId, prelude::*};

use crate::GameStates;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingResources {
            texture_atlas_handles: Vec::new(),
            handles: Vec::new(),
        })
        .insert_resource(TilemapAtlas { tilemap: None })
        .add_systems(OnEnter(GameStates::Loading), start_loading)
        .add_systems(Update, check_loaded.run_if(in_state(GameStates::Loading)))
        .add_systems(OnExit(GameStates::Loading), finish_loading);
    }
}

#[derive(Resource)]
struct LoadingResources {
    texture_atlas_handles: Vec<HandleUntyped>,
    handles: Vec<HandleUntyped>,
}

#[derive(Resource)]
pub struct TilemapAtlas {
    pub tilemap: Option<Handle<TextureAtlas>>,
}

fn start_loading(asset_server: Res<AssetServer>, mut loading_resources: ResMut<LoadingResources>) {
    let handles = asset_server.load_folder("tilemap").unwrap();
    loading_resources
        .handles
        .extend(handles.iter().map(|h| h.clone()));
    loading_resources
        .texture_atlas_handles
        .extend(handles.iter().map(|h| h.clone()));

    loading_resources
        .handles
        .push(asset_server.load_untyped("levels/level1/tilemap_ground.csv"));
    loading_resources
        .handles
        .push(asset_server.load_untyped("levels/level1/tileset.json"));
}

fn check_loaded(
    asset_server: Res<AssetServer>,
    loading_resources: Res<LoadingResources>,
    mut state: ResMut<NextState<GameStates>>,
    mut events: EventReader<AssetEvent<Image>>,
) {
    for _ in &mut events {
        match asset_server.get_group_load_state(
            loading_resources
                .handles
                .iter()
                .map(|h| h.id())
                .collect::<Vec<HandleId>>(),
        ) {
            bevy::asset::LoadState::Loaded => state.set(GameStates::Menu),
            _ => (),
        }
    }
}

fn finish_loading(
    loading_resources: Res<LoadingResources>,
    mut tilemap_atlas: ResMut<TilemapAtlas>,
    mut atlasses: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut atlas_builder = TextureAtlasBuilder::default();
    for tile in &loading_resources.texture_atlas_handles {
        let tile_image: Handle<Image> = tile.clone().typed();
        atlas_builder.add_texture(tile_image.clone(), textures.get(&tile_image).unwrap());
    }
    let atlas = atlas_builder.finish(&mut textures).unwrap();
    let atlas_handle = atlasses.add(atlas);
    tilemap_atlas.tilemap = Some(atlas_handle);
}
