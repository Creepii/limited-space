use bevy::prelude::*;

use crate::GameStates;

use super::character::{Character, CurrentCharacter};

#[derive(Component)]
struct CharacterIndicator {
    character: Character,
}

pub struct IndicatorPlugin;

impl Plugin for IndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Level), create_overlay);
        app.add_systems(
            Update,
            update_character_indicators.run_if(in_state(GameStates::Level)),
        );
    }
}

fn make_character_component(
    p: &mut ChildBuilder,
    background: &Handle<Image>,
    character_image: &Handle<Image>,
    number_style: &TextStyle,
    character: Character,
    number: usize,
) {
    p.spawn(NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|p| {
        p.spawn((
            ImageBundle {
                image: UiImage {
                    texture: background.clone(),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
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
            },
            CharacterIndicator { character },
        ))
        .with_children(|p| {
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
    let font: Handle<Font> = asset_server.load("fonts/NotoSans-Regular.ttf");
    let background = asset_server.load("characters/portrait_background.png");
    let turtle_face = asset_server.load("characters/turtle_face.png");
    let rabbit_face = asset_server.load("characters/rabbit_face.png");
    let number_style = TextStyle {
        font: font.clone(),
        font_size: 16.0,
        color: Color::BLACK,
    };
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
            make_character_component(
                p,
                &background,
                &turtle_face,
                &number_style,
                Character::Turtle,
                1,
            );
            make_character_component(
                p,
                &background,
                &rabbit_face,
                &number_style,
                Character::Rabbit,
                2,
            );
        });
}

fn update_character_indicators(
    character: Res<CurrentCharacter>,
    mut query: Query<(&mut BackgroundColor, &CharacterIndicator)>,
) {
    for (mut color, indicator) in &mut query {
        let is_selected = character.current == indicator.character;
        let character_color = indicator.character.color();
        let [h, s, l, a] = character_color.as_hsla_f32();
        let new_color = if is_selected {
            Color::hsla(h, s, l + 0.1, a)
        } else {
            character_color
        };
        *color = BackgroundColor(new_color);
    }
}
