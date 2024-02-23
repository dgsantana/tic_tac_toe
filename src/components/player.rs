use crate::components::Symbol;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::renet::ClientId;
use serde::{Deserialize, Serialize};

/// Contains player ID and it's playing symbol.
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    symbol: Symbol,
    replication: Replication,
}

impl PlayerBundle {
    pub fn new(client_id: ClientId, symbol: Symbol) -> Self {
        Self {
            player: Player(client_id),
            symbol,
            replication: Replication,
        }
    }

    /// Same as [`Self::new`], but with [`SERVER_ID`].
    pub fn server(symbol: Symbol) -> Self {
        Self::new(SERVER_ID, symbol)
    }
}

#[derive(Component, Serialize, Deserialize, Deref)]
pub struct Player(ClientId);

impl Player {
    pub fn client_id(&self) -> ClientId {
        self.0
    }
}