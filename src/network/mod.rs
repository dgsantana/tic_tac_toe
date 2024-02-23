mod client;
mod discovery;
mod server;

pub use client::*;
pub use discovery::*;
pub use server::*;

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::transport::{NetcodeClientTransport, NetcodeServerTransport},
};

use crate::{
    components::{CellIndex, Player, Symbol},
    events::CellPick,
    state::GameState,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_plugins(ReplicationPlugins);
            app.replicate::<Symbol>();
            app.replicate::<CellIndex>();
            app.replicate::<Player>();
            app.add_client_event::<CellPick>(EventType::Ordered);
            app.add_plugins(ClientNetworkPlugin);
            app.add_plugins(ServerNetworkPlugin);
            app.add_plugins(DiscoveryPlugin);
            app.add_systems(OnEnter(GameState::MainMenu), tear_down_network);
        }
    }
}

fn tear_down_network(
    mut commands: Commands,
    mut server: Option<ResMut<RenetServer>>,
    mut client: Option<ResMut<RenetClient>>,
    mut players: Option<ResMut<PlayersInGame>>,
    mut discovery_client_state: ResMut<NextState<DiscoveryClientState>>,
    mut discovery_server_state: ResMut<NextState<DiscoveryServerState>>,
) {
    if let Some(mut server) = server.take() {
        info!("tearing down server");
        server.disconnect_all();
    }
    if let Some(mut client) = client.take() {
        info!("tearing down client");
        client.disconnect();
    }
    if let Some(mut players) = players.take() {
        for &player_entity in players.players.iter() {
            commands.entity(player_entity).despawn_recursive();
        }
        players.players.clear();
        info!("tearing down players");
    }
    info!("tearing down network resources");
    discovery_client_state.set(DiscoveryClientState::Stopped);
    discovery_server_state.set(DiscoveryServerState::Stopped);
    commands.remove_resource::<RenetServer>();
    commands.remove_resource::<NetcodeServerTransport>();
    commands.remove_resource::<RenetClient>();
    commands.remove_resource::<NetcodeClientTransport>();
}
