use assets::{TileSetAssetLoader, TilesAssetLoader};
use bevy::{prelude::*, window::WindowResolution};
use gamelogic::{
    character::DiscoveredCharacters,
    level_mgr::{LevelManager, LoadedLevel, ManagedLevel},
    GameLogicPlugins,
};
use loading::{LoadingPlugin, TilemapAtlas};
use menu::MenuPlugin;
use tilemap::{TileSet, Tiles};

mod assets;
mod gamelogic;
mod loading;
mod menu;
mod physics;
mod tilemap;
mod util;

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
enum GameState {
    #[default]
    Loading,
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup_base)
        .add_systems(OnEnter(GameState::InGame), load_current_level)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        resolution: WindowResolution::new(1024.0, 640.0),
                        title: String::from("Limited Space"),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(LoadingPlugin)
        .add_plugins(GameLogicPlugins)
        .add_plugins(MenuPlugin)
        .add_asset::<Tiles>()
        .add_asset::<TileSet>()
        .init_asset_loader::<TilesAssetLoader>()
        .init_asset_loader::<TileSetAssetLoader>()
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup_base(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::default().with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..default()
        },
        MainCamera,
    ));
    commands.spawn(LevelManager {
        current: ManagedLevel::Level1,
    });
}

fn load_current_level(
    asset_server: Res<AssetServer>,
    tilemap_atlas: Res<TilemapAtlas>,
    atlasses: Res<Assets<TextureAtlas>>,
    tiles: Res<Assets<Tiles>>,
    tilesets: Res<Assets<TileSet>>,
    commands: Commands,
    query: Query<&LevelManager>,
) {
    let manager = query.single();
    manager.load_level(
        asset_server,
        tilemap_atlas,
        atlasses,
        tiles,
        tilesets,
        commands,
    );
}

fn unload_current_level(
    commands: Commands,
    query: Query<&LevelManager>,
    discovered: Query<&mut DiscoveredCharacters>,
    level_entities: Query<(Entity, &LoadedLevel)>,
) {
    let manager = query.single();
    manager.unload_level(commands, discovered, level_entities.iter());
}
