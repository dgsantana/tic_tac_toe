use bevy::prelude::*;

use crate::{
    network::{DiscoverServers, DiscoveryClientState, FoundNewServerEvent},
    resources::ServerConnectionInfo,
    state::GameState,
    ui::{BUTTON_BG_COLOR, HOVER_BG_COLOR},
    utils::tear_down_with_component,
};

pub struct ClientUiPlugin;

impl Plugin for ClientUiPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::Connect), setup_request_server_ip);
            app.add_systems(
                Update,
                (
                    update_server_ip.run_if(resource_changed::<ButtonInput<KeyCode>>),
                    handle_connect_button,
                )
                    .run_if(in_state(GameState::Connect)),
            );
            app.add_systems(
                Update,
                handle_new_server_found
                    .run_if(in_state(GameState::Connect))
                    .run_if(on_event::<FoundNewServerEvent>()),
            );
            app.add_systems(
                OnExit(GameState::Connect),
                tear_down_with_component::<ClientConfigRoot>,
            );
            app.add_systems(
                OnEnter(GameState::WaitingConnection),
                setup_waiting_connection,
            );
            app.add_systems(
                OnExit(GameState::WaitingConnection),
                tear_down_with_component::<ClientWaitingRoot>,
            );
        }
    }
}

#[derive(Component)]
struct ClientConfigRoot;

#[derive(Component)]
struct ClientWaitingRoot;

#[derive(Component)]
struct ServerIpTextEdit;

#[derive(Component)]
struct ConnectButton;

pub fn setup_request_server_ip(
    mut commands: Commands,
    connection: Res<ServerConnectionInfo>,
    mut discovery_client_state: ResMut<NextState<DiscoveryClientState>>,
) {
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..default()
    };
    let input_text_style = TextStyle {
        font_size: 40.0,
        color: Color::DARK_GRAY,
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
        .insert(ClientConfigRoot)
        .id();
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .set_parent(root)
        .id();
    commands
        .spawn(TextBundle {
            text: Text::from_section("Enter server IP address", text_style.clone()),
            ..default()
        })
        .set_parent(container);
    commands
        .spawn(TextBundle {
            text: Text::from_section(connection.server_addr.to_string(), input_text_style),
            style: Style {
                width: Val::Px(300.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .insert(ServerIpTextEdit)
        .set_parent(container);

    commands
        .spawn(ButtonBundle {
            style: Style {
                margin: UiRect::top(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .insert(ConnectButton)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("Connect", text_style),
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
        })
        .set_parent(container);
    // Start the discovery client
    discovery_client_state.set(DiscoveryClientState::Running);
}

fn update_server_ip(
    mut query: Query<(&ServerIpTextEdit, &mut Text)>,
    mut state: ResMut<NextState<GameState>>,
    mut connection: ResMut<ServerConnectionInfo>,
    input: Res<ButtonInput<KeyCode>>,
) {
    const VALID_KEYS: [(KeyCode, char); 22] = [
        (KeyCode::Digit0, '0'),
        (KeyCode::Digit1, '1'),
        (KeyCode::Digit2, '2'),
        (KeyCode::Digit3, '3'),
        (KeyCode::Digit4, '4'),
        (KeyCode::Digit5, '5'),
        (KeyCode::Digit6, '6'),
        (KeyCode::Digit7, '7'),
        (KeyCode::Digit8, '8'),
        (KeyCode::Digit9, '9'),
        (KeyCode::Period, '.'),
        (KeyCode::NumpadDecimal, '.'),
        (KeyCode::Numpad0, '0'),
        (KeyCode::Numpad1, '1'),
        (KeyCode::Numpad2, '2'),
        (KeyCode::Numpad3, '3'),
        (KeyCode::Numpad4, '4'),
        (KeyCode::Numpad5, '5'),
        (KeyCode::Numpad6, '6'),
        (KeyCode::Numpad7, '7'),
        (KeyCode::Numpad8, '8'),
        (KeyCode::Numpad9, '9'),
    ];
    for (_, mut text) in query.iter_mut() {
        let mut current_input_text = text.sections[0].value.clone();
        if input.just_pressed(KeyCode::Backspace) {
            current_input_text.pop();
        }
        for key in input.get_just_pressed() {
            if let Some((_, key)) = VALID_KEYS.iter().find(|(k, _)| k == key) {
                current_input_text.push_str(&key.to_string());
            }
        }
        if let Ok(ip) = current_input_text.parse() {
            connection.server_addr = ip;
        }
        text.sections[0].value = current_input_text;
        if input.just_pressed(KeyCode::Enter) {
            state.set(GameState::WaitingConnection);
        }
    }
}

fn handle_new_server_found(
    mut query: Query<(&ServerIpTextEdit, &mut Text)>,
    mut event: EventReader<FoundNewServerEvent>,
    mut connection: ResMut<ServerConnectionInfo>,
    _servers: Res<DiscoverServers>,
) {
    // If we want to use a text input to select the server
    for event in event.read() {
        for (_, mut text) in query.iter_mut() {
            let new_ip = event.server.to_string();
            text.sections[0].value = new_ip;
            connection.server_addr = event.server;
        }
    }

    // If we want to use a drop down menu to select the server
    // for server in servers.servers() {
    //     info!("Found server: {}", server);
    // }
}

fn handle_connect_button(
    mut connect_button: Query<
        (&ConnectButton, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut state: ResMut<NextState<GameState>>,
    mut discover_client_state: ResMut<NextState<DiscoveryClientState>>,
) {
    for (_, interaction, mut background_color) in connect_button.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = HOVER_BG_COLOR.into();
                state.set(GameState::WaitingConnection);
                // Stop the discovery client
                discover_client_state.set(DiscoveryClientState::Stopped);
            }
            Interaction::Hovered => {
                *background_color = HOVER_BG_COLOR.into();
            }
            Interaction::None => {
                *background_color = BUTTON_BG_COLOR.into();
            }
        }
    }
}

fn setup_waiting_connection(mut commands: Commands, connection: Res<ServerConnectionInfo>) {
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
        .insert(ClientWaitingRoot)
        .id();

    let container = commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .set_parent(root)
        .id();
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                format!("Connecting to server @ {}", connection.server_addr),
                text_style,
            ),
            ..default()
        })
        .set_parent(container);
}
