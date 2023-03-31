use crate::font::FontGlyph;
use crate::irect::IRect;

pub trait FontSheet {
    fn rgba_image(&self) -> &image::RgbaImage;
    fn rect_of(&self, glyph: FontGlyph) -> IRect;
}
