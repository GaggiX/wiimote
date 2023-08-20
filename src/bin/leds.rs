/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use std::{thread::sleep, time::Duration};
use wiimote_stuff::{Action, Wiimote};

fn main() {
    env_logger::init_from_env("LOG");
    let w = Wiimote::open();
    loop {
        for i in 0..4 {
            w.write(Action::PlayerLeds(1 << i));
            sleep(Duration::from_millis(100))
        }
    }
}
