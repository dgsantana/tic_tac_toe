mod components;
mod events;
mod game;
mod network;
mod resources;
mod state;
mod ui;
mod utils;

use crate::resources::ServerConnectionInfo;
use crate::state::GameState;
use bevy::prelude::*;
use resources::{CurrentTurn, SymbolFont, Winner};

const PROTOCOL_ID: u64 = 0;
const PORT: u16 = 5000;
const GRID_SIZE: usize = 3;
const LINES_COUNT: usize = GRID_SIZE + 1;
const CELL_SIZE: f32 = 100.0;
const LINE_THICKNESS: f32 = 10.0;
const BOARD_SIZE: f32 = CELL_SIZE * GRID_SIZE as f32 + LINES_COUNT as f32 * LINE_THICKNESS;
const BOARD_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BUTTON_SIZE: f32 = CELL_SIZE / 1.2;
const BUTTON_MARGIN: f32 = (CELL_SIZE + LINE_THICKNESS - BUTTON_SIZE) / 2.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(network::NetworkPlugin);

    app.init_state::<GameState>();
    app.init_resource::<ServerConnectionInfo>();
    app.init_resource::<CurrentTurn>();
    app.init_resource::<SymbolFont>();
    app.insert_resource(Winner::default());
    app.add_plugins(game::GamePlugin);
    app.add_plugins(ui::MenuPlugin);

    app.add_systems(Startup, setup_camera);
    app.add_systems(
        Update,
        return_to_main_menu.run_if(not(in_state(GameState::MainMenu))),
    );
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn return_to_main_menu(
    mut state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard_input.get_just_pressed() {
        if let KeyCode::Escape = key {
            state.set(GameState::MainMenu);
            break;
        }
    }
}
