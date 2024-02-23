use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    client_connected,
    prelude::*,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig,
    },
};

use crate::{
    components::{Player, Symbol},
    resources::{CurrentTurn, ServerConnectionInfo},
    state::GameState,
    utils::any_component_added,
    PORT, PROTOCOL_ID,
};

pub struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(
                OnEnter(GameState::WaitingConnection),
                start_connection.map(|r| {
                    if let Err(e) = r {
                        error!("Failed to start connection: {}", e);
                    }
                }),
            );
            app.add_systems(
                Update,
                client_start_game
                    .run_if(client_connected)
                    .run_if(any_component_added::<Player>),
            );
        }
    }
}

/// Runs on the client side to connect to the server.
fn start_connection(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
    server_config: Res<ServerConnectionInfo>,
) -> anyhow::Result<()> {
    let server_channels_config = network_channels.get_server_configs();
    let client_channels_config = network_channels.get_client_configs();

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;
    let server_addr = SocketAddr::new(server_config.server_addr, PORT);
    info!("connecting to server at {}", server_addr);
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))
        .inspect_err(|e| error!("Failed to create udp socket. {e}"))?;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .inspect_err(|e| error!("Failed on netcode client transport. {e}"))?;

    commands.insert_resource(client);
    commands.insert_resource(transport);
    // Set the current turn to black.
    commands.insert_resource(CurrentTurn::new(Symbol::Cross));
    Ok(())
}

fn client_start_game(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}
