use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

use crate::{
    physics::{Collider, CollisionBox},
    GameState,
};

use super::{
    level::{GoalFlag, PushButton},
    level_mgr::LevelManager,
};

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            switch_characters.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            trigger_meet_character.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            trigger_push_buttons.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(Update, trigger_flag.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, player_movement.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, PartialEq, Eq, Debug, Clone)]
pub enum Character {
    Turtle,
    Rabbit,
    Crocodile,
}

impl Character {
    pub fn color(&self) -> Color {
        match self {
            Character::Turtle => Color::BEIGE,
            Character::Rabbit => Color::AQUAMARINE,
            Character::Crocodile => Color::BLUE,
        }
    }

    pub fn face_texture(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            Character::Turtle => asset_server.load("characters/turtle_face.png"),
            Character::Rabbit => asset_server.load("characters/rabbit_face.png"),
            Character::Crocodile => asset_server.load("characters/crocodile_face.png"),
        }
    }

    pub fn texture(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            Character::Turtle => asset_server.load("characters/turtle.png"),
            Character::Rabbit => asset_server.load("characters/rabbit.png"),
            Character::Crocodile => asset_server.load("characters/crocodile.png"),
        }
    }

    pub fn collision_box(&self) -> CollisionBox {
        CollisionBox::Circle { radius: 18.0 }
    }
}

#[derive(Component)]
pub struct CurrentCharacter {
    pub current: Character,
}

#[derive(Component)]
pub struct DiscoveredCharacters {
    pub discovered: Vec<Character>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub current: CurrentCharacter,
    pub discovered: DiscoveredCharacters,
}

fn trigger_flag(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut flags: Query<(&CollisionBox, &Transform, &mut GoalFlag)>,
    mut manager: Query<&mut LevelManager>,
    characters: Query<(&CollisionBox, &Transform, &Character), Without<GoalFlag>>,
) {
    for (goal_box, goal_trafo, mut flag) in &mut flags {
        let mut all_collided = true;
        for (character_box, character_trafo, _character) in &characters {
            let is_colliding = Collider::collide(
                &character_box
                    .to_collider(character_trafo.translation.x, character_trafo.translation.y),
                &goal_box.to_collider(goal_trafo.translation.x, goal_trafo.translation.y),
            );
            if !is_colliding {
                all_collided = false;
            }
        }
        let prev_reached = flag.reached;
        if !prev_reached && all_collided {
            flag.reached = true;
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/win.ogg"),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new_absolute(1.0),
                    speed: 1.0,
                    paused: false,
                },
            });
            if let Some(next) = flag.next_level {
                manager.single_mut().next = Some(next);
            }
        }
    }
}

fn trigger_push_buttons(
    asset_server: Res<AssetServer>,
    mut buttons: Query<(
        &CollisionBox,
        &Transform,
        &mut PushButton,
        &mut Handle<Image>,
    )>,
    characters: Query<(&CollisionBox, &Transform, &Character), Without<PushButton>>,
) {
    for (button_box, button_trafo, mut button, mut button_texture) in &mut buttons {
        let mut any_collided = false;
        for (character_box, character_trafo, _character) in &characters {
            let is_colliding = Collider::collide(
                &character_box
                    .to_collider(character_trafo.translation.x, character_trafo.translation.y),
                &button_box.to_collider(button_trafo.translation.x, button_trafo.translation.y),
            );
            if is_colliding {
                any_collided = true;
            }
        }
        let prev_pressed = button.pressed;
        if prev_pressed != any_collided {
            button.pressed = any_collided;
            *button_texture = if any_collided {
                asset_server.load("tilemap/push_button_pressed.png")
            } else {
                asset_server.load("tilemap/push_button.png")
            };
        }
    }
}

fn switch_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&DiscoveredCharacters, &mut CurrentCharacter)>,
) {
    // switch characters
    let number_keys = [
        KeyCode::Key0,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];
    for (number, key) in number_keys.into_iter().skip(1).enumerate() {
        if keys.just_pressed(key) {
            for (discovered, mut current) in &mut query {
                let selected_character = discovered.discovered.get(number);
                if let Some(selected) = selected_character {
                    let prev_character = &current.current;
                    if prev_character != selected {
                        commands.spawn(AudioBundle {
                            source: asset_server.load("sounds/switch_character.ogg"),
                            settings: PlaybackSettings {
                                mode: PlaybackMode::Despawn,
                                volume: Volume::new_absolute(1.0),
                                speed: 1.0,
                                paused: false,
                            },
                        });
                        current.current = selected.clone();
                    }
                }
            }
        }
    }
}

fn on_meet_character(
    first: &Character,
    second: &Character,
    mut discovered: Mut<DiscoveredCharacters>,
    current: &CurrentCharacter,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let new_character = if first == &current.current {
        if !discovered.discovered.contains(second) {
            Some(second)
        } else {
            None
        }
    } else if second == &current.current {
        if !discovered.discovered.contains(first) {
            Some(first)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_character) = new_character {
        info!("New character: {:?}", new_character);
        commands.spawn(AudioBundle {
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Absolute(VolumeLevel::new(1.0)),
                speed: 1.0,
                paused: false,
            },
            source: asset_server.load("sounds/new_character.ogg"),
        });
        discovered.discovered.push(new_character.clone());
    }
}

fn trigger_meet_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Character, &CollisionBox, &Transform)>,
    mut player_query: Query<(&mut DiscoveredCharacters, &CurrentCharacter)>,
) {
    let collidables: Vec<(&Character, &CollisionBox, &Transform)> = query.iter().collect();
    for i in 0..collidables.len() {
        for j in i + 1..collidables.len() {
            let collider_a = {
                let (_, collision_box, transform) = collidables[i];
                collision_box.to_collider(transform.translation.x, transform.translation.y)
            };
            let collider_b = {
                let (_, collision_box, transform) = collidables[j];
                collision_box.to_collider(transform.translation.x, transform.translation.y)
            };
            if Collider::collide(&collider_a, &collider_b) {
                let (discovered, current) = player_query.single_mut();
                on_meet_character(
                    collidables[i].0,
                    collidables[j].0,
                    discovered,
                    current,
                    &mut commands,
                    &asset_server,
                );
            }
        }
    }
}

const PLAYER_SPEED: f32 = 256.0;

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    player_query: Query<&CurrentCharacter>,
    mut query: Query<(&Character, &mut Transform)>,
) {
    if let Ok(current) = player_query.get_single() {
        for (character, mut transform) in &mut query {
            if &current.current == character {
                let direction = Vec2::new(
                    if keys.pressed(KeyCode::A) {
                        -1.0
                    } else if keys.pressed(KeyCode::D) {
                        1.0
                    } else {
                        0.0
                    },
                    if keys.pressed(KeyCode::W) {
                        1.0
                    } else if keys.pressed(KeyCode::S) {
                        -1.0
                    } else {
                        0.0
                    },
                );
                let movement =
                    direction.normalize_or_zero() * (PLAYER_SPEED * time.delta_seconds());
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
                let view_rotation = Vec2::Y.angle_between(movement);
                if !view_rotation.is_nan() {
                    transform.rotation = transform.rotation.lerp(
                        Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, view_rotation),
                        0.2,
                    );
                }
            }
        }
    }
}
