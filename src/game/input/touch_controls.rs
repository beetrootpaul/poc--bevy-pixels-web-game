use std::ops::{Add, Div, Sub};

use bevy::math::ivec2;
use bevy::prelude::*;

use crate::game::player::PlayerMovement;
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
    is_pressed: bool,
    desired_player_movement: PlayerMovement,
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
                    is_pressed: false,
                    desired_player_movement: PlayerMovement::Up,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(distance, 0))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_RIGHT,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_RIGHT_PRESSED,
                    is_pressed: false,
                    desired_player_movement: PlayerMovement::Right,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(0, distance))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_DOWN,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_DOWN_PRESSED,
                    is_pressed: false,
                    desired_player_movement: PlayerMovement::Down,
                },
            });

            commands.spawn(TouchButtonBundle {
                position: Position(area_center.add(ivec2(-distance, 0))),
                touch_button: TouchButton {
                    sprite_regular: SpriteSheet::TOUCH_BUTTON_LEFT,
                    sprite_pressed: SpriteSheet::TOUCH_BUTTON_LEFT_PRESSED,
                    is_pressed: false,
                    desired_player_movement: PlayerMovement::Left,
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
            let sprite = match touch_button.is_pressed {
                true => &touch_button.sprite_pressed,
                false => &touch_button.sprite_regular,
            };
            let wh = sprite.sheet_rect.1.sub(sprite.sheet_rect.0);
            pixel_canvas.draw_sprite(
                position.0.sub(wh.div(2)),
                &sprite_sheet.touch_controls,
                sprite.sheet_rect,
            );
        }
    }

    pub fn handle_touch_input(
        touches: Res<Touches>,
        pixel_canvas: Res<PixelCanvas>,
        mut touch_buttons_query: Query<(&mut TouchButton, &Position)>,
        mut player_movement_query: Query<&mut PlayerMovement>,
    ) {
        for (mut touch_button, position) in touch_buttons_query.iter_mut() {
            let sprite = &touch_button.sprite_regular;
            let wh = sprite.sheet_rect.1.sub(sprite.sheet_rect.0);
            let touchable_xy1 = position.0.sub(wh.div(2));
            let touchable_xy2 = position.0.add(wh.div(2));

            touch_button.is_pressed = false;

            for touch in touches.iter() {
                let touch_xy =
                    pixel_canvas.real_viewport_xy_to_logical_canvas_xy(touch.position().as_ivec2());
                if touch_xy.cmpge(touchable_xy1).all() && touch_xy.cmple(touchable_xy2).all() {
                    touch_button.is_pressed = true;
                    for mut player_movement in player_movement_query.iter_mut() {
                        *player_movement = touch_button.desired_player_movement;
                    }
                }
            }
        }
    }
}
