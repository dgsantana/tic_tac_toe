use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::transport::NetcodeClientTransport};

use crate::{resources::Winner, state::GameState, utils::tear_down_with_component};

pub struct WinnerPlugin;

impl Plugin for WinnerPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::GameOver), setup_winner_ui);
            app.add_systems(
                OnExit(GameState::GameOver),
                tear_down_with_component::<WinnerRoot>,
            );
        }
    }
}

#[derive(Component)]
struct WinnerRoot;

fn setup_winner_ui(
    mut commands: Commands,
    winner: Res<Winner>,
    client_transport: Option<Res<NetcodeClientTransport>>,
) {
    let current_player = client_transport
        .as_ref()
        .map(|client| client.client_id())
        .unwrap_or(SERVER_ID);

    let game_over_message = match &winner.client_id() {
        Some(winner) => {
            if *winner == current_player {
                "You won!".to_string()
            } else {
                "You lost!".to_string()
            }
        }
        None => "It's a draw!".to_string(),
    };

    debug!("Game over message: {}", game_over_message);

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
        .insert((WinnerRoot, Name::new("UIRoot")))
        .id();

    let container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::GRAY.with_a(0.5).into(),
            ..Default::default()
        })
        .set_parent(root)
        .id();
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                game_over_message,
                TextStyle {
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        })
        .set_parent(container);
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Press 'ESC' to return to the main menu.",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..default()
                },
            ),
            ..default()
        })
        .set_parent(container);
}
