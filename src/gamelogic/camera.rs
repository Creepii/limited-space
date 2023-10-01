use bevy::prelude::*;

use crate::{util::Lerp, GameStates, MainCamera};

use super::character::{Character, CurrentCharacter};

enum CameraMode {
    AllCharacters,
    CurrentCharacter,
}

#[derive(Resource)]
struct CurrentCameraMode {
    current: CameraMode,
}

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentCameraMode {
            current: CameraMode::AllCharacters,
        });
        app.add_systems(
            Update,
            switch_camera_mode.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(Update, camera_movement.run_if(in_state(GameStates::Level)));
    }
}

impl CurrentCameraMode {
    fn toggle(&mut self) {
        match self.current {
            CameraMode::AllCharacters => self.current = CameraMode::CurrentCharacter,
            CameraMode::CurrentCharacter => self.current = CameraMode::AllCharacters,
        }
    }
}

fn switch_camera_mode(keys: Res<Input<KeyCode>>, mut camera_mode: ResMut<CurrentCameraMode>) {
    // switch camera mode
    if keys.just_pressed(KeyCode::F) {
        camera_mode.toggle();
    }
}

const CAMERA_SCALE: f32 = 0.4;
const CAMERA_PADDING: f32 = 128.0;

fn camera_movement(
    character: Res<CurrentCharacter>,
    camera_mode: Res<CurrentCameraMode>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<MainCamera>>,
        Query<&Window>,
        Query<(&Character, &Transform)>,
    )>,
) {
    let (x, y, scale) = match camera_mode.current {
        CameraMode::AllCharacters => {
            let character_transforms: Vec<Transform> =
                param_set.p2().iter().map(|c| c.1.clone()).collect();
            let ((min_x, max_x), (min_y, max_y)) = character_transforms
                .iter()
                .map(|t| (t.translation.x, t.translation.y))
                .fold(
                    ((f32::MAX, f32::MIN), (f32::MAX, f32::MIN)),
                    |(px, py), (x, y)| ((x.min(px.0), x.max(px.1)), (y.min(py.0), y.max(py.1))),
                );
            let size_x = (min_x - max_x).abs() + CAMERA_PADDING;
            let size_y = (min_y - max_y).abs() + CAMERA_PADDING;
            let (window_width, window_height) = {
                fn get_size(window: &Window) -> (f32, f32) {
                    (window.width(), window.height())
                }
                get_size(param_set.p1().single())
            };
            let scale = (size_x / window_width).max(size_y / window_height);
            let scale = scale.max(CAMERA_SCALE);
            let center = Vec2::new((max_x + min_x) / 2.0, (max_y + min_y) / 2.0);
            (center.x, center.y, scale)
        }
        CameraMode::CurrentCharacter => {
            let character_transform = param_set
                .p2()
                .iter()
                .filter(|c| c.0 == &character.current)
                .map(|c| c.1)
                .next()
                .unwrap()
                .clone();
            (
                character_transform.translation.x,
                character_transform.translation.y,
                CAMERA_SCALE,
            )
        }
    };
    for mut camera_transform in param_set.p0().iter_mut() {
        camera_transform.translation.x = camera_transform.translation.x.lerp(x, 0.3);
        camera_transform.translation.y = camera_transform.translation.y.lerp(y, 0.3);
        camera_transform.scale.x = camera_transform.scale.x.lerp(scale, 0.3);
        camera_transform.scale.y = camera_transform.scale.y.lerp(scale, 0.3);
    }
}
