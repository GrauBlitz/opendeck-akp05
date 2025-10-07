use mirajazz::{error::MirajazzError, types::DeviceInput};

use crate::mappings::{ENCODER_COUNT, KEY_COUNT};

pub fn process_input(input: u8, state: u8) -> Result<DeviceInput, MirajazzError> {
    log::debug!("Processing input: {}, {}", input, state);

    match input {
        (0..=10) | (64..=67) | 56 | 57 => read_button_press(input, state),
        160 | 161 | 80 | 81 | 144 | 145 | 112 | 113 => read_encoder_value(input),
        51 | 53 | 54 | 55 => read_encoder_press(input, state),
        _ => Err(MirajazzError::BadData),
    }
}

fn read_button_states(states: &[u8]) -> Vec<bool> {
    let mut bools = vec![];

    for i in 0..KEY_COUNT {
        bools.push(states[i + 1] != 0);
    }

    bools
}

fn read_button_press(input: u8, mut state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut button_states = vec![0x01];
    button_states.extend(vec![0u8; KEY_COUNT + 1]);

    if input == 0 {
        return Ok(DeviceInput::ButtonStateChange(read_button_states(
            &button_states,
        )));
    }

    log::debug!("Incoming input: {:#?}", input);
    let pressed_index: usize = match input {
        // 10 buttons with displays
        (1..=10) => input as usize,
        // 4 "buttons" on the touch screen, they only send state 0
        (64..=67) => (input) as usize - 53,
        // TODO: Swiping actions, they only send state 0 and do not appear as buttons currently
        56 => 15, // From left to right
        57 => 16, // From right to left
        _ => return Err(MirajazzError::BadData),
    };
    // TODO: Handle single-state touchscreen button
    // if pressed_index >= 11 && pressed_index <= 14 {
    //     state = 1;
    // }
    log::debug!("Corrected pressed index: {:#?}", pressed_index);

    button_states[pressed_index] = state;

    Ok(DeviceInput::ButtonStateChange(read_button_states(
        &button_states,
    )))
}

fn read_encoder_value(input: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_values = vec![0i8; ENCODER_COUNT];

    let (encoder, value): (usize, i8) = match input {
        // Left most encoder
        160 => (0, -1),
        161 => (0, 1),
        // Middle left encoder
        80 => (1, -1),
        81 => (1, 1),
        // Middle right encoder
        144 => (2, -1),
        145 => (2, 1),
        // Right most encoder
        112 => (3, -1),
        113 => (3, 1),
        _ => return Err(MirajazzError::BadData),
    };

    encoder_values[encoder] = value;
    Ok(DeviceInput::EncoderTwist(encoder_values))
}

// TODO: Encoders only send a pushed state with state = 1 but no released state
fn read_encoder_press(input: u8, mut state: u8) -> Result<DeviceInput, MirajazzError> {
    let mut encoder_states = vec![false; ENCODER_COUNT];

    let encoder: usize = match input {
        // Left most encoder
        55 => 0,
        // Middle left encoder
        53 => 1,
        // Middle right encoder
        51 => 2,
        // Right most encoder
        54 => 3,
        _ => return Err(MirajazzError::BadData),
    };

    encoder_states[encoder] = state != 0;
    Ok(DeviceInput::EncoderStateChange(encoder_states))
}
