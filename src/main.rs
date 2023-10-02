use assets::{TileSetAssetLoader, TilesAssetLoader};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowResolution,
};
use gamelogic::{
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
        .add_systems(Update, level_loading.run_if(in_state(GameState::InGame)))
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
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
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
        current: None,
        next: Some(ManagedLevel::Level1),
    });
}

fn level_loading(
    level_entities: Query<(Entity, &LoadedLevel)>,
    asset_server: Res<AssetServer>,
    tilemap_atlas: Res<TilemapAtlas>,
    atlasses: ResMut<Assets<TextureAtlas>>,
    tiles: Res<Assets<Tiles>>,
    tilesets: Res<Assets<TileSet>>,
    mut commands: Commands,
    mut camera: Query<&mut Transform, With<MainCamera>>,
    mut query: Query<&mut LevelManager, Changed<LevelManager>>,
) {
    let mut camera = camera.single_mut();
    if let Ok(mut manager) = query.get_single_mut() {
        manager.unload_level(&mut commands, level_entities.iter());
        manager.load_level(
            asset_server,
            tilemap_atlas,
            atlasses,
            tiles,
            tilesets,
            &mut camera,
            commands,
        );
        manager.current = manager.next;
        manager.next = None;
    }
}
