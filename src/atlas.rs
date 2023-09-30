use bevy::{asset::HandleId, prelude::*};

use crate::GameStates;

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Loading), start_loading_atlas)
            .add_systems(
                Update,
                check_atlas_loaded.run_if(in_state(GameStates::Loading)),
            )
            .add_systems(OnExit(GameStates::Loading), build_atlas);
    }
}

#[derive(Component)]
struct AtlasFolder(Vec<HandleUntyped>);

fn start_loading_atlas(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = asset_server.load_folder("tilemap").unwrap();
    commands.spawn(AtlasFolder(handles));
}

fn check_atlas_loaded(
    asset_server: Res<AssetServer>,
    query: Query<&AtlasFolder>,
    mut state: ResMut<NextState<GameStates>>,
    mut events: EventReader<AssetEvent<Image>>,
) {
    for handles in &query {
        for _ in &mut events {
            match asset_server
                .get_group_load_state(handles.0.iter().map(|h| h.id()).collect::<Vec<HandleId>>())
            {
                bevy::asset::LoadState::Loaded => state.set(GameStates::Menu),
                _ => (),
            }
        }
    }
}

fn build_atlas(
    query: Query<&AtlasFolder>,
    mut atlasses: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    for folder in &query {
        let mut atlas_builder = TextureAtlasBuilder::default();
        for tile in &folder.0 {
            let tile_image: Handle<Image> = tile.clone().typed();
            atlas_builder.add_texture(tile_image.clone(), textures.get(&tile_image).unwrap());
        }
        let atlas = atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = atlasses.add(atlas);
    }
}
