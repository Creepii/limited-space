use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};

use crate::{
    physics::{CollisionBox, Solid},
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
        app.add_systems(
            Update,
            update_animations.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(Update, player_movement.run_if(in_state(GameState::InGame)));
    }
}

fn update_animations(
    time: Res<Time>,
    mut query: Query<(
        &AnimationFrames,
        &Walking,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (frames, walking, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() && walking.walking {
            let current_index = sprite.index;
            let next_index = (current_index + 1) % frames.frames;
            sprite.index = next_index;
        }
    }
}

#[derive(Component, PartialEq, Eq, Debug, Clone)]
pub enum Character {
    Turtle,
    Rabbit,
    Crocodile,
    Lizard,
}

#[derive(Component)]
pub struct AnimationFrames {
    pub frames: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Character {
    pub fn color(&self) -> Color {
        match self {
            Character::Turtle => Color::hsl(73.0, 0.7, 0.75),
            Character::Rabbit => Color::hsl(340.0, 0.55, 0.85),
            Character::Lizard => Color::hsl(47.0, 0.9, 0.7),
            Character::Crocodile => Color::BLUE,
        }
    }

    pub fn face_texture(&self, asset_server: &Res<AssetServer>) -> Handle<Image> {
        match self {
            Character::Turtle => asset_server.load("characters/turtle_face.png"),
            Character::Rabbit => asset_server.load("characters/rabbit_face.png"),
            Character::Crocodile => asset_server.load("characters/crocodile_face.png"),
            Character::Lizard => asset_server.load("characters/lizard_face.png"),
        }
    }

    pub fn texture(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlasses: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let texture_handle = match self {
            Character::Turtle => asset_server.load("characters/turtle_walk.png"),
            Character::Rabbit => asset_server.load("characters/rabbit_walk.png"),
            Character::Crocodile => asset_server.load("characters/crocodile_walk.png"),
            Character::Lizard => asset_server.load("characters/lizard_walk.png"),
        };
        let tile_size = match self {
            Character::Turtle => Vec2::new(32.0, 32.0),
            Character::Rabbit => Vec2::new(32.0, 32.0),
            Character::Crocodile => Vec2::new(32.0, 64.0),
            Character::Lizard => Vec2::new(32.0, 32.0),
        };
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, tile_size, 1, self.frames(), None, None);
        texture_atlasses.add(texture_atlas)
    }

    pub fn collision_box(&self) -> CollisionBox {
        match self {
            Character::Turtle => CollisionBox::Circle { radius: 18.0 },
            Character::Rabbit => CollisionBox::Circle { radius: 18.0 },
            Character::Crocodile => CollisionBox::Circle { radius: 18.0 },
            Character::Lizard => CollisionBox::Circle { radius: 10.0 },
        }
    }

    pub fn frames(&self) -> usize {
        match self {
            Character::Turtle => 4,
            Character::Rabbit => 4,
            Character::Crocodile => 4,
            Character::Lizard => 4,
        }
    }

    fn speed(&self, time: &Res<Time>) -> f32 {
        match self {
            Character::Turtle => 48.0,
            Character::Rabbit => 16.0 + 96.0 * (time.elapsed_seconds() * 6.0).sin().abs(),
            Character::Crocodile => 64.0,
            Character::Lizard => 96.0,
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
            let character_collider = &character_box
                .to_collider(character_trafo.translation.x, character_trafo.translation.y);
            let is_colliding = character_collider.does_collide(
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
            let character_collider = &character_box
                .to_collider(character_trafo.translation.x, character_trafo.translation.y);
            let is_colliding = character_collider.does_collide(
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
    mut characters: Query<&mut Walking>,
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
                        for mut walking in &mut characters {
                            walking.walking = false;
                        }
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

#[derive(Component)]
pub struct Walking {
    pub walking: bool,
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
            if collider_a.does_collide(&collider_b) {
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

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    player_query: Query<&CurrentCharacter>,
    solid_collider_query: Query<(&CollisionBox, &Transform, &Solid)>,
    mut query: Query<(&Character, &CollisionBox, &mut Walking, &mut Transform), Without<Solid>>,
) {
    if let Ok(current) = player_query.get_single() {
        for (character, collision_box, mut walking, mut transform) in &mut query {
            if current.current == *character {
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
                if direction.length_squared() == 0.0 {
                    walking.walking = false;
                    continue;
                }
                walking.walking = true;
                let movement =
                    direction.normalize_or_zero() * (character.speed(&time) * time.delta_seconds());
                // high speed leads to glitching because movement code isn't in fixed update
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
                let character_collider =
                    collision_box.to_collider(transform.translation.x, transform.translation.y);
                let mut total_penetration = Vec2::ZERO;
                solid_collider_query
                    .iter()
                    .filter(|(_, _, solid)| solid.whitelisted != Some(character.clone()))
                    .for_each(|(solid_collision_box, solid_transform, _)| {
                        let solid_collider = solid_collision_box.to_collider(
                            solid_transform.translation.x,
                            solid_transform.translation.y,
                        );

                        match character_collider.collide(&solid_collider) {
                            Some(penetration) => {
                                if penetration.is_finite() {
                                    total_penetration += penetration;
                                }
                            }
                            None => (),
                        }
                    });
                transform.translation -= total_penetration.extend(0.0);
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
