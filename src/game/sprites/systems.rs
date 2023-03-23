use bevy::prelude::*;

use crate::game::sprites::SpriteSheet;

pub struct SpritesSystems;

impl SpritesSystems {
    pub fn load_sprite_sheet(mut commands: Commands) {
        let spritesheet_bytes_main = include_bytes!("../../../assets/spritesheet.png");
        let spritesheet_bytes_touch_controls = include_bytes!("../../../assets/touch_controls.png");
        commands.insert_resource(SpriteSheet {
            main: image::load_from_memory(spritesheet_bytes_main)
                .expect("should load main spritesheet from memory")
                .to_rgba8(),
            touch_controls: image::load_from_memory(spritesheet_bytes_touch_controls)
                .expect("should load touch controls spritesheet from memory")
                .to_rgba8(),
        });
    }
}
