use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, ServerEvent,
    },
};

use crate::{
    components::{Player, PlayerBundle, Symbol},
    resources::CurrentTurn,
    state::GameState,
    PORT, PROTOCOL_ID,
};

use super::DiscoveryServerState;

pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::Hotseat), start_hotseat_game);
            app.add_systems(
                OnEnter(GameState::HostingLobby),
                start_listening.map(Result::unwrap),
            );
            app.add_systems(
                Update,
                server_handle_events.run_if(resource_exists::<RenetServer>),
            );
        }
    }
}

/// Tracks the players in the game on the server side.
#[derive(Resource)]
pub(super) struct PlayersInGame {
    pub players: Vec<Entity>,
}

impl PlayersInGame {
    fn new(players: &[Entity]) -> Self {
        Self {
            players: players.to_vec(),
        }
    }

    fn add_player(&mut self, player: Entity) {
        self.players.push(player);
    }
}

fn start_hotseat_game(mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
    let player1 = commands.spawn(PlayerBundle::server(Symbol::Cross)).id();
    let player2 = commands.spawn(PlayerBundle::server(Symbol::Nought)).id();
    commands.insert_resource(PlayersInGame::new(&[player1, player2]));
    state.set(GameState::Playing);
}

/// Runs on the server side to listen for incoming connections.
fn start_listening(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
    mut discovery_state_server: ResMut<NextState<DiscoveryServerState>>,
) -> anyhow::Result<()> {
    let server_channels_config = network_channels.get_server_configs();
    let client_channels_config = network_channels.get_client_configs();

    let server = RenetServer::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    // We use the 0.0.0.0 (UNSPECIFIED)  address to listen on all available network interfaces.
    let listen_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), PORT);
    info!("listening for connections at {}", listen_addr);
    let socket = UdpSocket::bind(listen_addr)?;
    let server_config = ServerConfig {
        current_time,
        max_clients: 1,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![listen_addr],
    };
    let transport = NetcodeServerTransport::new(server_config, socket)?;

    commands.insert_resource(server);
    commands.insert_resource(transport);

    let symbol = Symbol::Cross;
    let player = commands.spawn(PlayerBundle::server(symbol)).id();
    commands.insert_resource(PlayersInGame::new(&[player]));
    // Set the current turn to black.
    commands.insert_resource(CurrentTurn::new(symbol));
    // Start the discovery server.
    discovery_state_server.set(DiscoveryServerState::Running);
    Ok(())
}

fn server_handle_events(
    mut commands: Commands,
    mut server_events: EventReader<ServerEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut players_in_game: ResMut<PlayersInGame>,
    mut discovery_state_server: ResMut<NextState<DiscoveryServerState>>,
    players: Query<&Symbol, With<Player>>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("client connected: {}", client_id);
                let server_player = players.single();
                let stone = server_player.next();
                let player = commands.spawn(PlayerBundle::new(*client_id, stone)).id();
                players_in_game.add_player(player);
                discovery_state_server.set(DiscoveryServerState::Stopped);
                game_state.set(GameState::Playing);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("client disconnected: {} with reason: {}", client_id, reason);
                for &player_entity in players_in_game.players.iter() {
                    commands.entity(player_entity).despawn_recursive();
                }
                players_in_game.players.clear();
                game_state.set(GameState::Disconnected);
            }
        }
    }
}
