use bevy::math::{vec2, Vec2};
use bevy::prelude::{Component, Query, Res, ResMut};

use crate::game::position::Position;
use crate::game::GameArea;
use crate::irect::IRect;
use crate::pico8::Pico8Color;
use crate::pixel_canvas::PixelCanvas;

#[derive(Component)]
pub struct HitCircle {
    pub r: f32,
    pub offset: Vec2,
}

pub struct CollisionSystems;

impl CollisionSystems {
    pub fn draw_hit_circles(
        query: Query<(&HitCircle, &Position)>,
        game_area: Res<GameArea>,
        mut pixel_canvas: ResMut<PixelCanvas>,
    ) {
        for (hit_circle, position) in query.iter() {
            let xy = game_area.game_area_xy_from(position.0);
            pixel_canvas.draw_ellipse(
                IRect {
                    left_top: xy + hit_circle.offset.round().as_ivec2(),
                    size: vec2(hit_circle.r * 2.0, hit_circle.r * 2.0).round().as_ivec2(),
                },
                Pico8Color::Red.into(),
            )
        }
    }
}
