use bevy::{
    app::AppExit,
    prelude::*,
    text::{Text, TextStyle},
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Style, UiRect, Val},
    utils::default,
};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(Update, button_interactions);
        app.add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component, Debug)]
enum ButtonKinds {
    Start,
    Quit,
}

impl ButtonKinds {
    fn get_color(&self, interaction: &Interaction) -> Color {
        match self {
            ButtonKinds::Start => Color::Hsla {
                hue: 103.0,
                saturation: 0.7,
                lightness: match interaction {
                    Interaction::Pressed => 0.6,
                    Interaction::Hovered => 0.5,
                    Interaction::None => 0.4,
                },
                alpha: 1.0,
            },
            ButtonKinds::Quit => Color::Hsla {
                hue: 5.0,
                saturation: 0.8,
                lightness: match interaction {
                    Interaction::Pressed => 0.7,
                    Interaction::Hovered => 0.6,
                    Interaction::None => 0.5,
                },
                alpha: 1.0,
            },
        }
    }
}

fn button_interactions(
    mut query: Query<(&Interaction, &mut BackgroundColor, &ButtonKinds), Changed<Interaction>>,
    mut exit: EventWriter<AppExit>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color, button) in &mut query {
        *bg_color = BackgroundColor(button.get_color(interaction));
        match *interaction {
            Interaction::Pressed => match button {
                ButtonKinds::Start => {
                    state.set(GameState::InGame);
                }
                ButtonKinds::Quit => {
                    exit.send(AppExit);
                }
            },
            _ => (),
        }
    }
}

#[derive(Component)]
struct MenuParent;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let title_style = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: Color::BLACK,
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    };

    info!("Setting up game menu");
    commands
        .spawn((
            ImageBundle {
                style: Style {
                    width: bevy::ui::Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("menu/title_screen.png"),
                    ..default()
                },
                ..default()
            },
            MenuParent,
        ))
        .with_children(|p| {
            p.spawn(TextBundle {
                text: Text::from_section("Puzzle Pawz", title_style.clone()),
                style: Style {
                    padding: UiRect::all(Val::Px(64.0)),
                    ..default()
                },
                ..default()
            });
            p.spawn((
                ButtonBundle {
                    style: Style {
                        border: UiRect::axes(Val::Px(5.0), Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        width: Val::Percent(30.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                ButtonKinds::Start,
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Start", button_text_style.clone()));
            });
            p.spawn((
                ButtonBundle {
                    style: Style {
                        border: UiRect::axes(Val::Px(5.0), Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        width: Val::Percent(30.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                ButtonKinds::Quit,
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
            });
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuParent>>) {
    info!("Destroying game menu");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
