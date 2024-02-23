use bevy::prelude::*;

#[derive(States, Clone, Copy, Debug, Eq, Hash, PartialEq, Default, Reflect)]
pub enum GameState {
    #[default]
    MainMenu,
    Connect,
    HostingLobby,
    WaitingConnection,
    Hotseat,
    Playing,
    GameOver,
    Draw,
    Disconnected,
}
