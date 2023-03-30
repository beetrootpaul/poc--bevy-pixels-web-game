use bevy::prelude::*;

use crate::game::position::Position;
use crate::game::GameArea;
use crate::irect::{irect, IRect};
use crate::pico8::Pico8Color;
use crate::pixel_canvas::PixelCanvas;

#[derive(Component)]
pub struct TrailSource {
    counter_to_spawn: i32,
}

impl TrailSource {
    const FRAMES_BETWEEN_SPAWNS: i32 = 6;

    pub fn new() -> Self {
        Self {
            counter_to_spawn: Self::FRAMES_BETWEEN_SPAWNS,
        }
    }

    fn update(&mut self) -> ShouldSpawnParticle {
        if self.counter_to_spawn <= 0 {
            self.counter_to_spawn = Self::FRAMES_BETWEEN_SPAWNS;
            ShouldSpawnParticle::Yes
        } else {
            self.counter_to_spawn -= 1;
            ShouldSpawnParticle::No
        }
    }
}

enum ShouldSpawnParticle {
    Yes,
    No,
}

#[derive(Component)]
pub struct TrailParticle {
    ttl: i32,
    xy: IVec2,
}

impl TrailParticle {
    const TTL_MAX: i32 = 28;
    const R_MAX: i32 = 2;

    fn new(xy: IVec2) -> Self {
        Self {
            ttl: Self::TTL_MAX,
            xy,
        }
    }

    fn update(&mut self) {
        self.ttl = 0.max(self.ttl - 1)
    }

    fn bounding_rect(&self) -> IRect {
        let ttl = f64::from(self.ttl);
        let ttl_max = f64::from(Self::TTL_MAX);
        let r_max = f64::from(Self::R_MAX);
        let r = ((ttl / ttl_max) * (r_max + 0.9)).floor() as i32;

        irect(2 * r + 1, 2 * r + 1).at(self.xy.x - r, self.xy.y - r)
    }

    fn should_disappear(&self) -> bool {
        self.ttl <= 0
    }
}

pub struct TrailSystems;

impl TrailSystems {
    pub fn update_trails(
        mut sources_query: Query<(&mut TrailSource, &Position)>,
        mut commands: Commands,
    ) {
        for (mut source, position) in sources_query.iter_mut() {
            if let ShouldSpawnParticle::Yes = source.update() {
                commands.spawn(TrailParticle::new(position.0));
            }
            source.counter_to_spawn -= 1;
        }
    }

    pub fn update_particles(
        mut particles_query: Query<(Entity, &mut TrailParticle)>,
        mut commands: Commands,
    ) {
        for (particle_entity, mut particle) in particles_query.iter_mut() {
            if particle.should_disappear() {
                commands.entity(particle_entity).despawn_recursive();
            } else {
                particle.update();
            }
        }
    }

    pub fn draw_particles(
        particles_query: Query<&TrailParticle>,
        mut pixel_canvas: ResMut<PixelCanvas>,
        game_area: Res<GameArea>,
    ) {
        for particle in particles_query.iter() {
            pixel_canvas.draw_ellipse_filled(
                game_area.game_area_rect_from(particle.bounding_rect()),
                Pico8Color::DarkGreen.into(),
            );
        }
    }
}
