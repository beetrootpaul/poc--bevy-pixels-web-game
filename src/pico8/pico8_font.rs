use bevy::prelude::{Commands, Resource};

use crate::font::{FontGlyph, FontSheet};
use crate::irect::{irect, IRect};

pub struct Pico8FontSystems;

impl Pico8FontSystems {
    pub fn load_font_sheet(mut commands: Commands) {
        let spritesheet_image = include_bytes!("../../assets/pico-8-font.png");
        commands.insert_resource(Pico8FontSheet {
            image: image::load_from_memory(spritesheet_image)
                .expect("should load font spritesheet from memory")
                .to_rgba8(),
        });
    }
}

#[derive(Resource)]
pub struct Pico8FontSheet {
    image: image::RgbaImage,
}

impl Pico8FontSheet {
    const CELLS_PER_ROW: i32 = 16;
    const CELL_W: i32 = 8;
    const CELL_H: i32 = 8;
    const GLYPH_W: i32 = 3;
    const GLYPH_H: i32 = 5;
}

impl FontSheet for Pico8FontSheet {
    fn rgba_image(&self) -> &image::RgbaImage {
        &self.image
    }

    fn rect_of(&self, glyph: FontGlyph) -> IRect {
        let cell_index: i32 = match glyph {
            FontGlyph::None => 0,
            FontGlyph::Digit0 => 48,
            FontGlyph::Digit1 => 49,
            FontGlyph::Digit2 => 50,
            FontGlyph::Digit3 => 51,
            FontGlyph::Digit4 => 52,
            FontGlyph::Digit5 => 53,
            FontGlyph::Digit6 => 54,
            FontGlyph::Digit7 => 55,
            FontGlyph::Digit8 => 56,
            FontGlyph::Digit9 => 57,
            FontGlyph::Colon => 58,
            FontGlyph::A => 97,
            FontGlyph::B => 98,
            FontGlyph::C => 99,
            FontGlyph::D => 100,
            FontGlyph::E => 101,
            FontGlyph::F => 102,
            FontGlyph::G => 103,
            FontGlyph::H => 104,
            FontGlyph::I => 105,
            FontGlyph::J => 106,
            FontGlyph::K => 107,
            FontGlyph::L => 108,
            FontGlyph::M => 109,
            FontGlyph::N => 110,
            FontGlyph::O => 111,
            FontGlyph::P => 112,
            FontGlyph::Q => 113,
            FontGlyph::R => 114,
            FontGlyph::S => 115,
            FontGlyph::T => 116,
            FontGlyph::U => 117,
            FontGlyph::V => 118,
            FontGlyph::W => 119,
            FontGlyph::X => 120,
            FontGlyph::Y => 121,
            FontGlyph::Z => 122,
        };
        irect(Self::GLYPH_W, Self::GLYPH_H).at(
            Self::CELL_W * (cell_index % Self::CELLS_PER_ROW),
            Self::CELL_H * (cell_index / Self::CELLS_PER_ROW),
        )
    }
}
