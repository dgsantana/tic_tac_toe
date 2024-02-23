use bevy::prelude::*;

use crate::{
    resources::{CurrentTurn, SymbolFont},
    state::GameState,
    ui::{FONT_SIZE, SYMBOL_SECTION},
    utils::tear_down_with_component,
    TEXT_COLOR,
};

pub struct TurnUiPlugin;

impl Plugin for TurnUiPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::Playing), setup_turn_ui);
            app.add_systems(
                PostUpdate,
                symbol_turn_text_system
                    .run_if(resource_changed::<CurrentTurn>)
                    .run_if(in_state(GameState::Playing)),
            );
            app.add_systems(
                OnExit(GameState::Playing),
                tear_down_with_component::<TurnUiRoot>,
            );
        }
    }
}

#[derive(Component)]
struct TurnUiRoot;

#[derive(Component)]
struct BottomText;

fn setup_turn_ui(mut commands: Commands, symbol_font: Res<SymbolFont>) {
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TurnUiRoot)
        .id();
    commands
        .spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Current turn: ",
                    TextStyle {
                        font_size: FONT_SIZE,
                        color: TEXT_COLOR,
                        ..Default::default()
                    },
                ),
                TextSection::new(
                    String::new(),
                    TextStyle {
                        font: symbol_font.clone(),
                        font_size: FONT_SIZE,
                        ..Default::default()
                    },
                ),
            ]),
            BottomText,
        ))
        .set_parent(container);
}

fn symbol_turn_text_system(
    mut bottom_text: Query<&mut Text, With<BottomText>>,
    current_turn: Res<CurrentTurn>,
) {
    let symbol_section = &mut bottom_text.single_mut().sections[SYMBOL_SECTION];
    symbol_section.value = current_turn.symbol().glyph().into();
    symbol_section.style.color = current_turn.symbol().color();
}
