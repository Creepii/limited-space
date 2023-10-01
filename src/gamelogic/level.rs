use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{physics::CollisionBox, GameState};

#[derive(Component, Debug)]
pub struct PushButton {
    pub pressed: bool,
    pub index: usize,
}

#[derive(Bundle)]
pub struct PushButtonBundle {
    button: PushButton,
    collision: CollisionBox,
    #[bundle()]
    sprite: SpriteBundle,
}

fn play_button_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Ref<PushButton>>,
) {
    for button_ref in &query {
        if button_ref.is_changed() && !button_ref.is_added() {
            commands.spawn(AudioBundle {
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new_absolute(1.0),
                    speed: 1.0,
                    paused: false,
                },
                source: asset_server.load("sounds/button.ogg"),
            });
        }
    }
}

pub fn create_push_button(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    transform: Transform,
    index: usize,
    color: Color,
) {
    let button_base = asset_server.load("tilemap/push_button_base.png");
    let button = asset_server.load("tilemap/push_button.png");
    commands
        .spawn(PushButtonBundle {
            button: PushButton {
                pressed: false,
                index,
            },
            collision: CollisionBox::Circle { radius: 4.0 },
            sprite: SpriteBundle {
                transform,
                sprite: Sprite { color, ..default() },
                texture: button,
                ..default()
            },
        })
        .with_children(|p| {
            p.spawn(SpriteBundle {
                texture: button_base,
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            play_button_sound.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::InGame), start_music);
    }
}

fn start_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/music.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new_absolute(1.0),
            speed: 1.0,
            paused: false,
        },
    });
}
