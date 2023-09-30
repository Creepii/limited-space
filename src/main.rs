use assets::{TileSetAssetLoader, TilesAssetLoader};
use bevy::{prelude::*, window::WindowResolution};
use level::LevelPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use tilemap::{TileSet, Tiles};

mod assets;
mod level;
mod loading;
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
        .add_plugins(LevelPlugin)
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
}
