use std::net::{IpAddr, Ipv4Addr, UdpSocket};

use bevy::prelude::*;

use crate::utils::inspect_and_log_error;

pub struct DiscoveryPlugin;

impl Plugin for DiscoveryPlugin {
    fn build(&self, app: &mut App) {
        {
            app.init_state::<DiscoveryServerState>();
            app.init_state::<DiscoveryClientState>();
            app.add_event::<FoundNewServerEvent>();
            app.insert_resource(DiscoverServers::default());
            app.add_systems(
                OnEnter(DiscoveryServerState::Running),
                start_discovery_server
                    .map(inspect_and_log_error)
                    .run_if(not(resource_exists::<DiscoveryServer>)),
            );
            app.add_systems(
                Update,
                handle_server_messages
                    .run_if(in_state(DiscoveryServerState::Running))
                    .run_if(resource_exists::<DiscoveryServer>),
            );
            app.add_systems(OnExit(DiscoveryServerState::Running), stop_discovery_server);
            app.add_systems(
                OnEnter(DiscoveryClientState::Running),
                start_discovery_client
                    .map(inspect_and_log_error)
                    .run_if(not(resource_exists::<DiscoveryClient>)),
            );
            app.add_systems(
                Update,
                handle_client_messages
                    .run_if(in_state(DiscoveryClientState::Running))
                    .run_if(resource_exists::<DiscoveryClient>),
            );
            app.add_systems(OnExit(DiscoveryClientState::Running), stop_discovery_client);
        }
    }
}

// The message sent by the client to discover servers, customize to your needs
const CLIENT_MESSAGE: &str = "TIC_TAC_TOE_DISCOVER";
// The message sent by the server in response to a discovery message, customize to your needs
const SERVER_MESSAGE: &str = "TIC_TAC_TOE_FOUND";
// The port used for discovery
const DISCOVER_PORT: u16 = 53005;


/// The state of the discovery server
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum DiscoveryServerState {
    #[default]
    Stopped,
    Running,
}

/// The discovery server
#[derive(Debug, Resource)]
struct DiscoveryServer {
    socket: UdpSocket,
}

/// Starts the discovery server
fn start_discovery_server(mut commands: Commands) -> anyhow::Result<()> {
    info!("Starting discovery");
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DISCOVER_PORT)).unwrap();
    socket.set_nonblocking(true)?;
    commands.insert_resource(DiscoveryServer { socket });
    Ok(())
}

/// Handles messages received by the discovery server
fn handle_server_messages(server: Res<DiscoveryServer>) {
    let mut buf = [0; 1024];
    match server.socket.recv_from(&mut buf) {
        Ok((size, addr)) => {
            let text = String::from_utf8_lossy(&buf[..size]);
            if text.starts_with(CLIENT_MESSAGE) {
                server
                    .socket
                    .send_to(SERVER_MESSAGE.as_bytes(), addr)
                    .unwrap();
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // WouldBlock is expected when there's no data available yet
        }
        Err(e) => {
            error!("Failed to receive message: {}", e);
        }
    }
}

/// Stops the discovery server
///
/// This just removes the resource, which will cause
/// the server to stop due to drop
fn stop_discovery_server(mut commands: Commands) {
    info!("Stopping discovery");
    commands.remove_resource::<DiscoveryServer>();
}

/// The state of the discovery client
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum DiscoveryClientState {
    #[default]
    Stopped,
    Running,
}

/// An event that is sent when a new server is found
#[derive(Debug, Event)]
pub struct FoundNewServerEvent {
    pub server: IpAddr,
}

/// The resource that holds the list of servers found
#[derive(Debug, Default, Resource)]
pub struct DiscoverServers {
    servers: Vec<IpAddr>,
}

impl DiscoverServers {
    /// Adds a server to the list of servers found
    fn add_server(&mut self, addr: IpAddr) -> bool {
        if !self.servers.contains(&addr) {
            self.servers.push(addr);
            true
        } else {
            false
        }
    }

    /// Returns an iterator over the servers found
    pub fn _servers(&self) -> impl Iterator<Item = &IpAddr> {
        self.servers.iter()
    }
}

/// The discovery client
#[derive(Debug, Resource)]
struct DiscoveryClient {
    socket: UdpSocket,
}

/// Starts the discovery client
fn start_discovery_client(mut commands: Commands) -> anyhow::Result<()> {
    info!("Starting discovery client");
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.set_nonblocking(true)?;
    socket.set_broadcast(true)?;
    // Send a discovery message to the broadcast address
    socket.send_to(CLIENT_MESSAGE.as_bytes(), (Ipv4Addr::BROADCAST, DISCOVER_PORT))?;
    commands.insert_resource(DiscoveryClient { socket });
    Ok(())
}

/// Handles messages received by the discovery client
fn handle_client_messages(
    client: Res<DiscoveryClient>,
    mut discover_servers: ResMut<DiscoverServers>,
    mut event: EventWriter<FoundNewServerEvent>,
) {
    let mut buf = [0; 1024];
    match client.socket.recv_from(&mut buf) {
        Ok((size, addr)) => {
            let text = String::from_utf8_lossy(&buf[..size]);
            if text.starts_with(SERVER_MESSAGE) && discover_servers.add_server(addr.ip()) {
                info!("Received discovery response from {}", addr);
                // Send an event to notify that a new server was found
                event.send(FoundNewServerEvent { server: addr.ip() });
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // WouldBlock is expected when there's no data available yet
        }
        Err(e) => {
            error!("Failed to receive message: {}", e);
        }
    }
    // Send another discovery message
    // If there are multiple servers, we keep storing them on the DiscoverServers resource.
    // This allows UI (not implemented yet) to show the list of servers found.
    client
        .socket
        .send_to(CLIENT_MESSAGE.as_bytes(), (Ipv4Addr::BROADCAST, DISCOVER_PORT))
        .inspect_err(|e| error!("Failed to send discovery request: {}", e))
        .ok();
}

/// Stops the discovery client
///
/// This just removes the resource, which will cause
/// the client to stop due to drop
fn stop_discovery_client(mut commands: Commands) {
    info!("Stopping discovery client");
    commands.remove_resource::<DiscoveryClient>()
}
