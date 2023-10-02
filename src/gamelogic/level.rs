use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    utils::HashSet,
};

use crate::{
    physics::{CollisionBox, Solid},
    GameState,
};

use super::level_mgr::ManagedLevel;

#[derive(Component, Debug)]
pub struct PushButton {
    pub pressed: bool,
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct GatedBridge {
    pub opened: bool,
    pub negated: bool,
    pub index: usize,
}

#[derive(Component)]
pub struct GoalFlag {
    pub next_level: Option<ManagedLevel>,
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
pub struct GatedBridgeBundle {
    pub bridge: GatedBridge,
    pub collision: CollisionBox,
    pub solid: Solid,
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

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            play_button_sound.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(Update, update_bridge.run_if(in_state(GameState::InGame)));
        app.add_systems(OnEnter(GameState::InGame), start_music);
    }
}

fn update_bridge(
    mut commands: Commands,
    query: Query<&PushButton>,
    mut bridge_query: Query<(Entity, &mut Visibility, &mut GatedBridge)>,
) {
    let pressed_indices: HashSet<usize> = query
        .iter()
        .filter(|b| b.pressed)
        .map(|b| b.index)
        .collect();
    for (entity, mut visible, mut bridge) in &mut bridge_query {
        let prev_opened = bridge.opened;
        let button_pressed = pressed_indices.contains(&bridge.index);
        let should_be_open = if bridge.negated {
            !button_pressed
        } else {
            button_pressed
        };
        if prev_opened != should_be_open {
            bridge.opened = should_be_open;
            match should_be_open {
                true => {
                    *visible = Visibility::Hidden;
                    commands.entity(entity).remove::<Solid>();
                }
                false => {
                    *visible = Visibility::Visible;
                    commands.entity(entity).insert(Solid { whitelisted: None });
                }
            }
            info!(
                "Bridge with index {} is now open={}",
                bridge.index, should_be_open
            );
        }
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
