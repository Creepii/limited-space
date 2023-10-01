use bevy::prelude::*;

use crate::GameStates;

#[derive(Component, PartialEq, Eq, Debug, Clone)]
pub enum Character {
    Turtle,
    Rabbit,
}

impl Character {
    pub fn color(&self) -> Color {
        match self {
            Character::Turtle => Color::BEIGE,
            Character::Rabbit => Color::AQUAMARINE,
        }
    }
}

#[derive(Resource)]
pub struct CurrentCharacter {
    pub current: Character,
}

#[derive(Resource)]
struct DiscoveredCharacters {
    discovered: Vec<Character>,
}

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentCharacter {
            current: Character::Rabbit,
        });
        app.insert_resource(DiscoveredCharacters {
            discovered: vec![Character::Turtle, Character::Rabbit],
        });
        app.add_systems(OnEnter(GameStates::Level), summon_characters);
        app.add_systems(
            Update,
            switch_characters.run_if(in_state(GameStates::Level)),
        );
        app.add_systems(Update, player_movement.run_if(in_state(GameStates::Level)));
    }
}

fn switch_characters(
    keys: Res<Input<KeyCode>>,
    discovered: Res<DiscoveredCharacters>,
    mut character: ResMut<CurrentCharacter>,
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
            let selected_character = discovered.discovered.get(number);
            if let Some(selected) = selected_character {
                character.current = selected.clone();
            }
        }
    }
}

fn summon_characters(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/turtle.png"),
            ..default()
        },
        Character::Turtle,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("characters/rabbit.png"),
            transform: Transform::from_xyz(0.0, 64.0, 0.0),
            ..default()
        },
        Character::Rabbit,
    ));
}

const PLAYER_SPEED: f32 = 256.0;

fn player_movement(
    time: Res<Time>,
    current: Res<CurrentCharacter>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Character, &mut Transform)>,
) {
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
