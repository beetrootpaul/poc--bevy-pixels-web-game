use bevy::prelude::*;

use crate::game::sprites::SpriteSheet;
use crate::game::xy::Xy;
use crate::game::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::pixel_canvas::PixelCanvas;

// TODO: fixed FPS
// TODO: adjust movement
const MOVEMENT_PER_FRAME: f32 = 1.;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    player_movement: PlayerMovement,
    xy: Xy,
    // TODO: ???
    // sprite_dimensions: SpriteDimensions,
    // hit_circle: HitCircle,
    // trail_origin: TrailOrigin,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub enum PlayerMovement {
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn there_is_no_player(query: Query<&Player>) -> bool {
        query.iter().count() == 0
    }

    pub fn spawn_player(
        mut commands: Commands,
        // TODO: ???
        // sprite_sheet: Res<SpriteSheet>,
        // meshes: ResMut<Assets<Mesh>>,
        // materials: ResMut<Assets<ColorMaterial>>,
        // fixed_fps_time: Res<FixedFpsTime>,
    ) {
        debug!("> SPAWN PLAYER");

        // TODO: ???
        // let hit_circle =
        //     // TODO: ??? HitCircle {
        // TODO: make r 3.5, then offset -0.3 in x for right facing movement and similar adjustments for other directions.
        //       Consider incorporating sprite dimensions to calculate sprite center (as an impl on a trait?)
        // r: 3.9,
        // offset: vec3(-0.5, 0.5, 0.),
        // };

        let initial_movement = PlayerMovement::Right;

        // TODO: ???
        // let mut parent_command = commands.spawn(PlayerBundle {
        commands.spawn(PlayerBundle {
            player: Player,
            player_movement: initial_movement.clone(),
            xy: Xy(1., 1.),
            // TODO: reorganize game area position calculations
            // TODO: add helpers for translating from window-centered coors to game area coords
            // TODO: ???
            // transform: Transform::from_xyz(0., -TOPBAR_H / 2., Z_LAYER_SPRITES_PLAYER),
            // sprite_dimensions: SpriteDimensions {
            //     padding_right: 1.,
            //     padding_bottom: 1.,
            // },
            // hit_circle: hit_circle.clone(),
            // TODO: express time in frames, instead of seconds maybe?
            // trail_origin: TrailOrigin::with_seconds_between_particles(
            //     4. * fixed_fps_time.duration.as_secs_f32(),
            // ),
        });

        // TODO: ???
        // #[cfg(debug_assertions)]
        // parent_command.with_children(|parent| {
        //     parent.spawn(create_hit_circle_visualization(
        //         &hit_circle,
        //         Z_LAYER_SPRITES_PLAYER,
        //         meshes,
        //         materials,
        //     ));
        // });
    }

    pub fn move_player(
        mut query: Query<(
            &PlayerMovement,
            &mut Xy,
            // TODO: ???
            // Option<&SpriteDimensions>
        )>,
    ) {
        // TODO: ???
        // TODO: is it possible to bind speed to FPS (change in FPS -> automatic change of speed to make it constant in result), without allowing for non-integers?
        // const MOVEMENT_PER_FRAME: f32 = 2.;

        for (player_movement, mut xy) in query.iter_mut() {
            match player_movement {
                PlayerMovement::Left => xy.0 -= MOVEMENT_PER_FRAME,
                PlayerMovement::Right => xy.0 += MOVEMENT_PER_FRAME,
                PlayerMovement::Up => xy.1 -= MOVEMENT_PER_FRAME,
                PlayerMovement::Down => xy.1 += MOVEMENT_PER_FRAME,
            }

            // TODO: consider having kind of a movement boundary component instead of referring to GAME_AREA_* directly here
            // TODO: implement clamp on entire Xy at once
            // TODO: better way for number type conversion?
            xy.0 = xy.0.clamp(0., (GAME_AREA_WIDTH - 1) as f32);
            xy.1 = xy.1.clamp(0., (GAME_AREA_HEIGHT - 1) as f32);

            // TODO: ???
            // let sprite_dimensions = maybe_sprite_dimensions
            //     .copied()
            //     .unwrap_or(SpriteDimensions::default());

            // TODO: ???
            // transform.translation.x = transform.translation.x.clamp(
            //     -GAME_AREA_W / 2. + sprite_dimensions.width / 2. - sprite_dimensions.padding_left,
            //     GAME_AREA_W / 2. - sprite_dimensions.width / 2. + sprite_dimensions.padding_right,
            // );
            // transform.translation.y = transform.translation.y.clamp(
            //     -GAME_AREA_H / 2. - TOPBAR_H / 2. + sprite_dimensions.height / 2.
            //         - sprite_dimensions.padding_bottom,
            //     GAME_AREA_H / 2. - TOPBAR_H / 2. - sprite_dimensions.height / 2.
            //         + sprite_dimensions.padding_top,
            // );
        }
    }

    // TODO: implement correct player sprite drawing instead of this TMP version
    pub fn draw_player(
        sprite_sheet: ResMut<SpriteSheet>,
        query: Query<(&Xy, &PlayerMovement), With<Player>>,
        mut pixel_canvas: ResMut<PixelCanvas>,
    ) {
        if let Some(rgba_image) = sprite_sheet.maybe_rgba_image.as_ref() {
            for (xy, player_movement) in query.iter() {
                // TODO: cache it
                let source_rect = SpriteSheet::source_rect_of_cell(
                    Self::get_sprite_index_for_movement(player_movement),
                );
                pixel_canvas.draw_sprite(xy, rgba_image, source_rect);
            }
        }
    }

    fn get_sprite_index_for_movement(movement: &PlayerMovement) -> usize {
        match *movement {
            PlayerMovement::Left => SpriteSheet::PLAYER_LEFT,
            PlayerMovement::Right => SpriteSheet::PLAYER_RIGHT,
            PlayerMovement::Up => SpriteSheet::PLAYER_UP,
            PlayerMovement::Down => SpriteSheet::PLAYER_DOWN,
        }
    }
}
