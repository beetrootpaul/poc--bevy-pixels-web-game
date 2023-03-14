use bevy::prelude::*;

use crate::game::sprites::SpriteSheet;

pub struct SpritesSystems;

impl SpritesSystems {
    pub fn load_sprite_sheet(mut commands: Commands) {
        let image_bytes = include_bytes!("../../../assets/spritesheet.png");
        let loaded_image: image::DynamicImage =
            image::load_from_memory(image_bytes).expect("should load image from memory");
        commands.insert_resource(SpriteSheet {
            maybe_rgba_image: Some(loaded_image.to_rgba8()),
        });
    }
}
