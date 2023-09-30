use bevy::prelude::*;

use crate::{atlas::TilemapAtlas, GameStates};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), setup_level);
        app.add_systems(OnExit(GameStates::Level), cleanup_level);
    }
}

fn setup_level(tilemap_atlas: Res<TilemapAtlas>, mut commands: Commands) {
    commands.spawn(SpriteSheetBundle {
        texture_atlas: tilemap_atlas.tilemap.as_ref().unwrap().clone(),
        ..default()
    });
}

fn cleanup_level() {}
