use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{CellIndex, Player, Symbol, SymbolBundle},
    events::CellPick,
    resources::{CurrentTurn, SymbolFont, Winner},
    state::GameState,
    utils::{any_component_added, local_player_turn, tear_down_with_component},
    BACKGROUND_COLOR, BOARD_COLOR, BOARD_SIZE, BUTTON_MARGIN, BUTTON_SIZE, CELL_SIZE, GRID_SIZE,
    LINES_COUNT, LINE_THICKNESS,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        {
            app.replicate::<GameElements>();

            app.add_systems(OnEnter(GameState::Playing), setup_game);
            app.add_systems(
                Update,
                (
                    cell_interaction_system.run_if(local_player_turn),
                    picking_system.run_if(has_authority),
                    symbol_init_system,
                    turn_advance_system.run_if(any_component_added::<CellIndex>),
                )
                    .chain_ignore_deferred()
                    .run_if(in_state(GameState::Playing)),
            );

            app.add_systems(
                OnExit(GameState::Playing),
                tear_down_with_component::<GameElements>,
            );
        }
    }
}

#[derive(Component, Serialize, Deserialize)]
struct GameElements;

#[derive(Component)]
struct GridNode;

fn setup_game(
    mut commands: Commands,
    mut winner: ResMut<Winner>,
    mut current_turn: ResMut<CurrentTurn>,
) {
    winner.clear();
    current_turn.reset();

    for line in 0..LINES_COUNT {
        let position =
            -BOARD_SIZE / 2.0 + line as f32 * (CELL_SIZE + LINE_THICKNESS) + LINE_THICKNESS / 2.0;

        // Horizontal
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::Y * position,
                    scale: Vec3::new(BOARD_SIZE, LINE_THICKNESS, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GameElements);

        // Vertical
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: BOARD_COLOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::X * position,
                    scale: Vec3::new(LINE_THICKNESS, BOARD_SIZE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GameElements);
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameElements)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Px(BOARD_SIZE - LINE_THICKNESS),
                        height: Val::Px(BOARD_SIZE - LINE_THICKNESS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            GridNode,
                            NodeBundle {
                                style: Style {
                                    display: Display::Grid,
                                    grid_template_columns: vec![GridTrack::auto(); GRID_SIZE],
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ))
                        .with_children(|parent| {
                            for _ in 0..GRID_SIZE * GRID_SIZE {
                                parent.spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Px(BUTTON_SIZE),
                                        height: Val::Px(BUTTON_SIZE),
                                        margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
                                        ..Default::default()
                                    },
                                    background_color: BACKGROUND_COLOR.into(),
                                    ..Default::default()
                                });
                            }
                        });
                });
        });
}

fn cell_interaction_system(
    mut buttons: Query<(Entity, &Parent, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    children: Query<&Children>,
    mut pick_events: EventWriter<CellPick>,
) {
    const HOVER_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);

    for (button_entity, button_parent, interaction, mut background) in &mut buttons {
        match interaction {
            Interaction::Pressed => {
                let buttons = children.get(**button_parent).unwrap();
                let index = buttons
                    .iter()
                    .position(|&entity| entity == button_entity)
                    .unwrap();

                // We send a pick event and wait for the pick to be replicated back to the client.
                // In case of server or single-player the event will re-translated into [`FromClient`] event to re-use the logic.
                pick_events.send(CellPick::new(index));
            }
            Interaction::Hovered => *background = HOVER_COLOR.into(),
            Interaction::None => *background = BACKGROUND_COLOR.into(),
        };
    }
}

/// Handles cell pick events.
///
/// Only for single-player and server.
fn picking_system(
    mut commands: Commands,
    mut pick_events: EventReader<FromClient<CellPick>>,
    symbols: Query<&CellIndex>,
    current_turn: Res<CurrentTurn>,
    players: Query<(&Player, &Symbol)>,
) {
    for FromClient { client_id, event } in pick_events.read().copied() {
        // It's good to check the received data, client could be cheating.
        if event.index() > GRID_SIZE * GRID_SIZE {
            debug!("received invalid cell index {:?}", event.index());
            continue;
        }

        if !players.iter().any(|(player, &symbol)| {
            player.client_id() == client_id && symbol == current_turn.symbol()
        }) {
            debug!(
                "player {client_id} chose cell {:?} at wrong turn",
                event.index()
            );
            continue;
        }

        if symbols
            .iter()
            .any(|cell_index| cell_index.index() == event.index())
        {
            debug!(
                "player {client_id} has chosen an already occupied cell {:?}",
                event.index()
            );
            continue;
        }

        // Spawn "blueprint" of the cell that client will replicate.
        commands
            .spawn(SymbolBundle::new(current_turn.symbol(), event.index()))
            .insert(GameElements);
    }
}

/// Initializes spawned symbol on client after replication and on server / single-player right after the spawn.
fn symbol_init_system(
    mut commands: Commands,
    symbol_font: Res<SymbolFont>,
    symbols: Query<(Entity, &CellIndex, &Symbol), Added<Symbol>>,
    grid_nodes: Query<&Children, With<GridNode>>,
    mut background_colors: Query<&mut BackgroundColor>,
) {
    for (symbol_entity, cell_index, symbol) in &symbols {
        let children = grid_nodes.single();
        let button_entity = *children
            .get(cell_index.index())
            .expect("symbols should point to valid buttons");

        let mut background = background_colors
            .get_mut(button_entity)
            .expect("buttons should be initialized with color");
        *background = BACKGROUND_COLOR.into();

        commands
            .entity(button_entity)
            .remove::<Interaction>()
            .add_child(symbol_entity);

        commands
            .entity(symbol_entity)
            .insert(TextBundle::from_section(
                symbol.glyph(),
                TextStyle {
                    font: symbol_font.clone(),
                    font_size: 80.0,
                    color: symbol.color(),
                },
            ));
    }
}

/// Checks the winner and advances the turn.
fn turn_advance_system(
    mut commands: Commands,
    mut current_turn: ResMut<CurrentTurn>,
    mut game_state: ResMut<NextState<GameState>>,
    players: Query<(&Player, &Symbol)>,
    symbols: Query<(&CellIndex, &Symbol)>,
) {
    let mut board = [None; GRID_SIZE * GRID_SIZE];
    for (cell_index, &symbol) in &symbols {
        board[cell_index.index()] = Some(symbol);
    }

    const WIN_CONDITIONS: [[usize; GRID_SIZE]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for indexes in WIN_CONDITIONS {
        let symbols = indexes.map(|index| board[index]);
        if symbols[0].is_some() && symbols.windows(2).all(|symbols| symbols[0] == symbols[1]) {
            game_state.set(GameState::GameOver);
            // Find the player with the winning symbol.
            let winner = players
                .iter()
                .find(|(_, &symbol)| symbol == symbols[0].unwrap())
                .map(|(player, _)| player.client_id());
            commands.insert_resource(Winner::new(winner));
            return;
        }
    }

    if board.iter().all(Option::is_some) {
        game_state.set(GameState::Draw);
    } else {
        current_turn.next();
    }
}
