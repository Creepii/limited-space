use assets::{TileSetAssetLoader, TilesAssetLoader};
use atlas::TilemapAtlasPlugin;
use bevy::{prelude::*, window::WindowResolution};
use level::LevelPlugin;
use menu::MenuPlugin;
use tilemap::{TileSet, Tiles};

mod assets;
mod atlas;
mod level;
mod menu;
mod tilemap;

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
enum GameStates {
    #[default]
    Loading,
    Menu,
    Level,
}

fn main() {
    App::new()
        .add_state::<GameStates>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup_base)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(1024.0, 640.0),
                title: String::from("Limited Space"),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TilemapAtlasPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(MenuPlugin)
        .add_asset::<Tiles>()
        .add_asset::<TileSet>()
        .init_asset_loader::<TilesAssetLoader>()
        .init_asset_loader::<TileSetAssetLoader>()
        .run();
}

fn setup_base(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
