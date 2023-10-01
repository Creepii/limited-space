use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    camera::CameraControlPlugin, character::CharacterPlugin, indicator::IndicatorPlugin,
    level::LevelPlugin,
};

mod camera;
pub mod character;
mod indicator;
mod level;
pub mod level_mgr;

pub struct GameLogicPlugins;

impl PluginGroup for GameLogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(LevelPlugin)
            .add(CharacterPlugin)
            .add(CameraControlPlugin)
            .add(IndicatorPlugin)
    }
}
