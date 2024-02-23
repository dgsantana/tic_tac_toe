use std::net::IpAddr;

use bevy::prelude::*;
use bevy_replicon::renet::ClientId;

use crate::components::Symbol;

/// Font to display unicode characters for [`Symbol`].
#[derive(Resource, Deref)]
pub struct SymbolFont(Handle<Font>);

impl FromWorld for SymbolFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("NotoEmoji-Regular.ttf"))
    }
}

/// Contains symbol to be used this turn.
#[derive(Resource, Default)]
pub struct CurrentTurn(Symbol);

impl CurrentTurn {
    pub fn new(symbol: Symbol) -> Self {
        Self(symbol)
    }

    pub fn symbol(&self) -> Symbol {
        self.0
    }

    pub fn next(&mut self) {
        self.0 = self.0.next();
    }

    pub fn reset(&mut self) {
        self.0 = Symbol::Cross;
    }
}

#[derive(Resource, Default, Deref)]
pub struct Winner(Option<ClientId>);

impl Winner {
    pub fn new(client_id: Option<ClientId>) -> Self {
        Self(client_id)
    }

    pub fn client_id(&self) -> Option<ClientId> {
        self.0
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }
}

#[derive(Resource)]
pub struct ServerConnectionInfo {
    pub server_addr: IpAddr,
}

impl Default for ServerConnectionInfo {
    fn default() -> Self {
        Self {
            server_addr: IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
        }
    }
}