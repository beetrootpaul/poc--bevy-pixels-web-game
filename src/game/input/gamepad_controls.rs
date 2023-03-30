use bevy::prelude::*;

use crate::game::player::PlayerMovement;

pub struct GamepadControlsSystems;

impl GamepadControlsSystems {
    const LEFT_STICK_THRESHOLD: f32 = 0.5;

    pub fn handle_gamepad_input(
        gamepads: Res<Gamepads>,
        gamepad_button_inputs: Res<Input<GamepadButton>>,
        gamepad_axis: Res<Axis<GamepadAxis>>,
        mut player_movement_query: Query<&mut PlayerMovement>,
    ) {
        for gamepad in gamepads.iter() {
            let left_stick_x = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
            let left_stick_y = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);
            let dpad_left = GamepadButton::new(gamepad, GamepadButtonType::DPadLeft);
            let dpad_right = GamepadButton::new(gamepad, GamepadButtonType::DPadRight);
            let dpad_up = GamepadButton::new(gamepad, GamepadButtonType::DPadUp);
            let button_south = GamepadButton::new(gamepad, GamepadButtonType::DPadDown);

            let left_stick_x_value = gamepad_axis.get(left_stick_x).unwrap_or(0.0);
            let left_stick_y_value = gamepad_axis.get(left_stick_y).unwrap_or(0.0);

            if left_stick_x_value < -Self::LEFT_STICK_THRESHOLD
                || gamepad_button_inputs.just_pressed(dpad_left)
            {
                for mut player_movement in player_movement_query.iter_mut() {
                    *player_movement = PlayerMovement::Left;
                }
            }
            if left_stick_x_value > Self::LEFT_STICK_THRESHOLD
                || gamepad_button_inputs.just_pressed(dpad_right)
            {
                for mut player_movement in player_movement_query.iter_mut() {
                    *player_movement = PlayerMovement::Right;
                }
            }
            if left_stick_y_value > Self::LEFT_STICK_THRESHOLD
                || gamepad_button_inputs.just_pressed(dpad_up)
            {
                for mut player_movement in player_movement_query.iter_mut() {
                    *player_movement = PlayerMovement::Up;
                }
            }
            if left_stick_y_value < -Self::LEFT_STICK_THRESHOLD
                || gamepad_button_inputs.just_pressed(button_south)
            {
                for mut player_movement in player_movement_query.iter_mut() {
                    *player_movement = PlayerMovement::Down;
                }
            }
        }
    }
}
