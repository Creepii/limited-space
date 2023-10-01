use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{physics::CollisionBox, GameState};

use super::level_mgr::ManagedLevel;

#[derive(Component, Debug)]
pub struct PushButton {
    pub pressed: bool,
    pub index: usize,
}

#[derive(Component)]
pub struct GoalFlag {
    pub next_level: ManagedLevel,
    pub reached: bool,
}

#[derive(Bundle)]
pub struct GoalFlagBundle {
    pub goal_flag: GoalFlag,
    pub collision: CollisionBox,
    #[bundle()]
    pub sprite: SpriteBundle,
}

#[derive(Bundle)]
pub struct PushButtonBundle {
    pub button: PushButton,
    pub collision: CollisionBox,
    #[bundle()]
    pub sprite: SpriteBundle,
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
