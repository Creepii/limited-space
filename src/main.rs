use bevy::{prelude::*, window::WindowResolution};
use menu::MenuPlugin;

mod menu;

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
enum GameStates {
    #[default]
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
        .add_plugins(MenuPlugin)
        .run();
}

fn setup_base(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
