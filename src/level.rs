use bevy::prelude::*;

use crate::GameStates;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), setup_level);
        app.add_systems(OnExit(GameStates::Level), cleanup_level);
    }
}

fn setup_level(mut commands: Commands) {}

fn cleanup_level() {}
