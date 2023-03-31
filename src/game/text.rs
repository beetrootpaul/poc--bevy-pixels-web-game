use bevy::prelude::*;

use crate::game::position::Position;
use crate::game::GameArea;
use crate::pico8::Pico8FontSheet;
use crate::pixel_canvas::PixelCanvas;

#[derive(Component)]
pub struct Text {
    pub content: String,
}

pub struct TextSystems;

impl TextSystems {
    pub fn draw_texts(
        font_sheet: Res<Pico8FontSheet>,
        query: Query<(&Text, &Position)>,
        game_area: Res<GameArea>,
        mut pixel_canvas: ResMut<PixelCanvas>,
    ) {
        for (text, position) in query.iter() {
            let xy = game_area.game_area_xy_from(position.0);
            pixel_canvas.draw_text(xy, font_sheet.as_ref(), &text.content);
        }
    }
}
