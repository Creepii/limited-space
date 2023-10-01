use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

use crate::{
    physics::{Collider, CollisionBox},
    GameStates,
};

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
    current: CurrentCharacter,
    discovered: DiscoveredCharacters,
}

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), summon_characters);
        app.add_systems(
            Update,
            switch_characters.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(
            Update,
            collect_characters.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(
            Update,
            character_collisions.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(Update, player_movement.run_if(in_state(GameStates::Level)));
    }
}

fn collect_characters(keys: Res<Input<KeyCode>>, mut query: Query<&mut DiscoveredCharacters>) {
    if keys.just_pressed(KeyCode::Z) {
        for mut discovered in &mut query {
            discovered.discovered.push(Character::Crocodile);
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

fn summon_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        current: CurrentCharacter {
            current: Character::Turtle,
        },
        discovered: DiscoveredCharacters {
            discovered: vec![Character::Turtle],
        },
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/turtle.png"),
            ..default()
        },
        Character::Turtle,
        CollisionBox::Circle { radius: 18.0 },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/rabbit.png"),
            transform: Transform::from_xyz(0.0, 64.0, 0.0),
            ..default()
        },
        Character::Rabbit,
        CollisionBox::Circle { radius: 18.0 },
    ));
}

fn on_character_collision(
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
            source: asset_server.load("sounds/coin.ogg"),
        });
        discovered.discovered.push(new_character.clone());
    }
}

fn character_collisions(
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
                on_character_collision(
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
    let current = player_query.single().current.clone();
    for (character, mut transform) in &mut query {
        if &current == character {
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
            let movement = direction.normalize_or_zero() * (PLAYER_SPEED * time.delta_seconds());
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
