use crate::game::position::Position;
use crate::game::text::Text;
use bevy::math::ivec2;
use bevy::prelude::{Bundle, Commands, Component, Query, With};

pub struct TopBarSystems;

impl TopBarSystems {
    pub fn there_is_no_top_bar(query: Query<(), With<TopBar>>) -> bool {
        query.iter().count() == 0
    }

    pub fn spawn_top_bar(mut commands: Commands) {
        commands.spawn(TopBarBundle {
            top_bar: TopBar,
            text: Text {
                content: "SCORE: 0".to_string(),
            },
            position: Position(ivec2(15, 5)),
        });
    }
}

#[derive(Component)]
pub struct TopBar;

#[derive(Bundle)]
struct TopBarBundle {
    top_bar: TopBar,
    text: Text,
    position: Position,
}
