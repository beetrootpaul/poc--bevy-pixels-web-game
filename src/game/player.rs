use bevy::prelude::*;

use crate::game::GameSystemSet;
use crate::game::xy::{xy, Xy};
use crate::pixel_art::PixelCanvas;

// TODO: fixed FPS
// TODO: adjust movement
const MOVEMENT_PER_FRAME: f32 = 0.04;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_player
                .run_if(there_is_no_player)
                .in_set(GameSystemSet::SpawnPlayer),
        );
        app.add_system(move_player.in_set(GameSystemSet::UpdateGame));
        app.add_system(draw_player.in_set(GameSystemSet::DrawPlayer));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    player_movement: PlayerMovement,
    xy: Xy,
    // TODO: ???
    // sprite_sheet_bundle: SpriteSheetBundle,
    // sprite_dimensions: SpriteDimensions,
    // hit_circle: HitCircle,
    // trail_origin: TrailOrigin,
}

#[derive(Component, Debug)]
pub enum PlayerMovement {
    Left,
    Right,
    Up,
    Down,
}

fn there_is_no_player(query: Query<&Player>) -> bool {
    query.iter().count() == 0
}

fn spawn_player(
    mut commands: Commands,
    // TODO: ???
    // sprite_sheet: Res<SpriteSheet>,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
    // fixed_fps_time: Res<FixedFpsTime>,
) {
    // TODO: ???
    // let hit_circle =
    //     // TODO: ??? HitCircle {
    // TODO: make r 3.5, then offset -0.3 in x for right facing movement and similar adjustments for other directions.
    //       Consider incorporating sprite dimensions to calculate sprite center (as an impl on a trait?)
    // r: 3.9,
    // offset: vec3(-0.5, 0.5, 0.),
    // };

    // TODO: ???
    // let mut parent_command = commands.spawn(PlayerBundle {
    commands.spawn(PlayerBundle {
        player: Player,
        player_movement: PlayerMovement::Right,
        xy: xy(1., 1.),
        // TODO: ???
        // sprite_sheet_bundle: SpriteSheetBundle {
        // TODO: reorganize game area position calculations
        // TODO: add helpers for translating from window-centered coors to game area coords
        // transform: Transform::from_xyz(0., -TOPBAR_H / 2., Z_LAYER_SPRITES_PLAYER),
        // texture_atlas: sprite_sheet.texture_atlas_handle.clone().unwrap(),
        // sprite: TextureAtlasSprite {
        //     index: get_sprite_index_for_movement(&initial_movement),
        //     anchor: Anchor::Center,
        //     ..default()
        // },
        // ..default()
        // },
        // sprite_dimensions: SpriteDimensions {
        //     padding_right: 1.,
        //     padding_bottom: 1.,
        //     ..default()
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

fn move_player(
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

    // TODO: ???
    // for (player_movement, mut transform, maybe_sprite_dimensions) in query.iter_mut() {
    for (player_movement, mut xy) in query.iter_mut() {
        match player_movement {
            PlayerMovement::Left => xy.0 -= MOVEMENT_PER_FRAME,
            PlayerMovement::Right => xy.0 += MOVEMENT_PER_FRAME,
            PlayerMovement::Up => xy.1 -= MOVEMENT_PER_FRAME,
            PlayerMovement::Down => xy.1 += MOVEMENT_PER_FRAME,
        }

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

// TODO: extract pixel drawing to a separate module
fn draw_player(mut pixel_canvas: ResMut<PixelCanvas>, query: Query<&Xy, With<Player>>) {
    for xy in query.iter() {
        if let Some(pixel_index) = pixel_canvas.pixel_index_at(xy) {
            // TODO: encapsulate frame access
            let frame = pixel_canvas.pixels.get_frame_mut();
            frame[4 * pixel_index] = 0xff;
            frame[4 * pixel_index + 1] = 0x00;
            frame[4 * pixel_index + 2] = 0x55;
            frame[4 * pixel_index + 3] = 0xff;
        }
    }
}
