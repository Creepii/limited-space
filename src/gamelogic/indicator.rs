use bevy::prelude::*;

use crate::GameState;

use super::{
    camera::{CameraMode, CurrentCameraMode},
    character::{Character, CurrentCharacter, DiscoveredCharacters},
};

#[derive(Component)]
struct CharacterIndicator {
    character: Character,
}

#[derive(Component)]
struct CharacterIndicatorParent;

#[derive(Component)]
struct CameraIndicator;

pub struct IndicatorPlugin;

impl Plugin for IndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_overlay);
        app.add_systems(
            Update,
            update_character_indicators.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            update_selected_indicators.run_if(in_state(GameState::InGame)),
        );
        app.add_systems(
            Update,
            update_camera_indicator.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_camera_indicator(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut UiImage, With<CameraIndicator>>,
    camera_mode: Res<CurrentCameraMode>,
) {
    for mut image in &mut query {
        image.texture = camera_mode.current.texture(&asset_server);
    }
}

fn update_character_indicators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    discovered: Query<&DiscoveredCharacters, Changed<DiscoveredCharacters>>,
    query: Query<(Entity, With<CharacterIndicatorParent>)>,
) {
    if let Ok(discovered) = discovered.get_single() {
        let font: Handle<Font> = asset_server.load("fonts/NotoSans-Regular.ttf");
        let background = asset_server.load("menu/portrait_background.png");
        let selected = asset_server.load("menu/portrait_selected.png");
        let number_style = TextStyle {
            font: font.clone(),
            font_size: 16.0,
            color: Color::BLACK,
        };
        let discovered = &discovered.discovered;
        info!("Updating indicators: {:?}", discovered);
        let (entity, _) = query.single();
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).with_children(|p| {
            for (i, character) in discovered.iter().enumerate() {
                make_character_component(
                    p,
                    &background,
                    &selected,
                    &character.face_texture(&asset_server),
                    &number_style,
                    character.clone(),
                    i + 1,
                );
            }
        });
    }
}

fn make_character_component(
    p: &mut ChildBuilder,
    background: &Handle<Image>,
    selected: &Handle<Image>,
    character_image: &Handle<Image>,
    number_style: &TextStyle,
    character: Character,
    number: usize,
) {
    p.spawn((NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        ..default()
    },))
        .with_children(|p| {
            p.spawn(ImageBundle {
                image: UiImage {
                    texture: background.clone(),
                    ..default()
                },
                background_color: BackgroundColor(character.color()),
                style: Style {
                    height: Val::Percent(100.0),
                    aspect_ratio: Some(1.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    ImageBundle {
                        image: UiImage {
                            texture: selected.clone(),
                            ..default()
                        },
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    CharacterIndicator { character },
                ));
                p.spawn(ImageBundle {
                    image: UiImage {
                        texture: character_image.clone(),
                        ..default()
                    },
                    style: Style {
                        width: Val::Percent(40.0),
                        height: Val::Percent(40.0),
                        ..default()
                    },
                    ..default()
                });
            });
            p.spawn(TextBundle {
                text: Text::from_section(number.to_string(), number_style.clone()),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(63.0),
                    left: Val::Px(17.0),
                    ..default()
                },
                ..default()
            });
        });
}

fn create_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(12.0),
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn((NodeBundle { ..default() }, CharacterIndicatorParent));
            p.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            });
            p.spawn((
                ImageBundle {
                    image: UiImage {
                        texture: CameraMode::CurrentCharacter.texture(&asset_server),
                        ..default()
                    },
                    ..default()
                },
                CameraIndicator,
            ));
        });
}

fn update_selected_indicators(
    character: Query<&CurrentCharacter>,
    mut query: Query<(&mut Visibility, &CharacterIndicator)>,
) {
    if let Ok(character) = character.get_single() {
        for (mut color, indicator) in &mut query {
            let is_selected = character.current == indicator.character;
            *color = if is_selected {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
