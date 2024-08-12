use std::{thread, time::Duration};

use rust_gamepad::gamepad::{self, Gamepad};

pub fn main() {
    // initalize gamepad - change device name for your system
    let js = Gamepad::new("/dev/input/js0", gamepad::XBOX_MAPPING.clone());
    // start background handler thread
    js.background_handler();
    // read initial gamepad state
    let mut last_state = gamepad::GamepadState::initial();

    loop {
        // get the gamepad buttons/sticks state
        let state = js.state();
        println!("state = {:?}", state);

        // determine if button was clicked
        let a_clicked = state.button_clicked(gamepad::Buttons::A, &last_state);
        if a_clicked {
            println!("A button clicked ... sleep for 3 seconds");
            thread::sleep(Duration::from_secs(3));
        }

        // read button state
        println!("a state is: {}", state.a());

        last_state = state;
    }
}
