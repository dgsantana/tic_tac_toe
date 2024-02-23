use bevy::prelude::*;

use crate::{state::GameState, utils::tear_down_with_component};

pub struct ServerUiPlugin;

impl Plugin for ServerUiPlugin {
    fn build(&self, app: &mut App) {
        {
            app.add_systems(OnEnter(GameState::HostingLobby), setup_hosting_lobby);
            app.add_systems(
                OnExit(GameState::HostingLobby),
                tear_down_with_component::<ServerWaitPlayerRoot>,
            );
        }
    }
}

#[derive(Component)]
pub struct ServerWaitPlayerRoot;

fn setup_hosting_lobby(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..default()
    };
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert((ServerWaitPlayerRoot, Name::new("UIRoot")))
        .id();

    commands
        .spawn(TextBundle {
            text: Text::from_section("Waiting for players to join...", text_style),
            ..Default::default()
        })
        .set_parent(root);
}
