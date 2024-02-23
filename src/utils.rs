use crate::components::{Player, Symbol};
use crate::resources::CurrentTurn;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon::renet::transport::NetcodeClientTransport;

/// Returns `true` if the local player can select cells.
pub fn local_player_turn(
    current_turn: Res<CurrentTurn>,
    client_transport: Option<Res<NetcodeClientTransport>>,
    players: Query<(&Player, &Symbol)>,
) -> bool {
    let client_id = client_transport
        .map(|client| client.client_id())
        .unwrap_or(SERVER_ID);

    players
        .iter()
        .any(|(player, &symbol)| player.client_id() == client_id && symbol == current_turn.symbol())
}

/// A condition for systems to check if any component of type `T` was added to the world.
pub fn any_component_added<T: Component>(components: Query<(), Added<T>>) -> bool {
    !components.is_empty()
}

/// Destroys all entities with the component `T`.
pub fn tear_down_with_component<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn inspect_and_log_error(result: Result<(), impl std::fmt::Debug>) {
    if let Err(e) = result {
        error!("Error: {:?}", e);
    }
}