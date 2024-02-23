use bevy::{app::AppExit, ecs::query::QueryData, prelude::*};

use crate::{state::GameState, utils::tear_down_with_component};

use super::{BUTTON_BG_COLOR, HOVER_BG_COLOR};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
            app.add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            );
            app.add_systems(
                OnExit(GameState::MainMenu),
                tear_down_with_component::<MainMenuRoot>,
            );
        }
    }
}

#[derive(Component)]
struct MainMenuRoot;

#[derive(Component)]
enum MenuButton {
    Hotseat,
    Host,
    Join,
    Quit,
}

pub fn setup_main_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Auto,
        margin: UiRect::all(Val::Auto),
        padding: UiRect::vertical(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..default()
    };

    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert((MainMenuRoot, Name::new("UIRoot")))
        .id();

    let buttons_container = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..Default::default()
        })
        .set_parent(root)
        .id();

    let buttons = vec![
        ("Hotseat", MenuButton::Hotseat),
        ("Host", MenuButton::Host),
        ("Join", MenuButton::Join),
        ("Quit", MenuButton::Quit),
    ];

    for (text, button) in buttons {
        commands
            .spawn(ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                let text_style = text_style.clone();
                parent.spawn(TextBundle {
                    text: Text::from_section(text, text_style),
                    ..Default::default()
                });
            })
            .insert(button)
            .set_parent(buttons_container);
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
struct HandleButtonQuery {
    interaction: &'static Interaction,
    button: &'static MenuButton,
    background_color: &'static mut BackgroundColor,
}

fn handle_main_menu_buttons(
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<HandleButtonQuery, Changed<Interaction>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for mut query in interaction_query.iter_mut() {
        match query.interaction {
            Interaction::Pressed => match query.button {
                MenuButton::Hotseat => {
                    state.set(GameState::Hotseat);
                }
                MenuButton::Host => {
                    state.set(GameState::HostingLobby);
                }
                MenuButton::Join => {
                    state.set(GameState::Connect);
                }
                MenuButton::Quit => {
                    app_exit_events.send(AppExit);
                }
            },
            Interaction::Hovered => {
                *query.background_color = HOVER_BG_COLOR.into();
            }
            _ => {
                *query.background_color = BUTTON_BG_COLOR.into();
            }
        }
    }
}
