use std::ops::{Add, Div, Sub};
use bevy::math::ivec2;

use bevy::prelude::*;

use crate::game::position::Position;
use crate::game::sprites::{Sprite, SpriteSheet};
use crate::game::GameArea;
use crate::pixel_canvas::PixelCanvas;

#[derive(Bundle)]
struct TouchButtonBundle {
    touch_button: TouchButton,
    position: Position,
}

#[derive(Component)]
pub struct TouchButton {
    sprite_regular: Sprite,
    sprite_pressed: Sprite,
}

pub struct TouchControlsSystems;

impl TouchControlsSystems {
    pub fn spawn_touch_controls(
        game_area: Res<GameArea>,
        touch_buttons_query: Query<Entity, With<TouchButton>>,
        mut commands: Commands,
    ) {
        if !game_area.is_changed() {
            return;
        }

        info!("DE-SPAWN touch controls");

        for touch_button_entity in touch_buttons_query.iter() {
            commands.entity(touch_button_entity).despawn_recursive();
        }

        info!("SPAWN touch controls");

        if let Some((xy1, xy2)) = game_area.touch_controls_area() {
            let area_center = xy1.add(xy2.sub(xy1).div(2));
            let distance = 28;

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(0, -distance))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_UP,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_UP_PRESSED,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(distance, 0))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_RIGHT,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_RIGHT_PRESSED,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(0, distance))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_DOWN,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_DOWN_PRESSED,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(-distance, 0))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_LEFT,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_LEFT_PRESSED,
                },
            });
        }
    }

    pub fn draw_touch_controls(
        query: Query<(&TouchButton, &Position)>,
        mut pixel_canvas: ResMut<PixelCanvas>,
        sprite_sheet: Res<SpriteSheet>,
    ) {
        for (touch_button, position) in query.iter() {
            let sprite = &touch_button.sprite_regular;
            let wh = sprite.sheet_rect.1.sub(sprite.sheet_rect.0);
            pixel_canvas.draw_sprite(
                position.0.sub(wh.div(2)),
                &sprite_sheet.touch_controls,
                sprite.sheet_rect,
            );
        }
    }
}
