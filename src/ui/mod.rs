mod client;
mod main;
mod server;
mod turn;
mod winner;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

// Bottom text defined in two sections, first for text and second for symbols with different font.
const SYMBOL_SECTION: usize = 1;

const FONT_SIZE: f32 = 40.0;
const BUTTON_BG_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const HOVER_BG_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, 0.5);

pub struct MenuPlugin;

impl PluginGroup for MenuPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(client::ClientUiPlugin)
            .add(main::MainMenuPlugin)
            .add(server::ServerUiPlugin)
            .add(turn::TurnUiPlugin)
            .add(winner::WinnerPlugin)
    }
}
