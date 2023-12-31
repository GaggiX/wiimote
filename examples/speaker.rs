/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use std::io::{stdin, Read};
use wiimote::{Action, Wiimote};

fn main() {
    env_logger::init_from_env("LOG");
    let w = Wiimote::open();

    w.write(Action::SpeakerEnable(true));
    w.write(Action::SpeakerMute(true));
    w.write_registers(0xa20009, &[0x01]);
    w.write_registers(0xa20001, &[0x08]);
    w.write_registers(0xa20001, &[0x00, 0x40, 0x70, 0x17, 0x60, 0x00, 0x00]);
    w.write_registers(0xa20008, &[0x01]);
    w.write(Action::SpeakerMute(false));
    loop {
        let mut buf = [0; 20];
        stdin().read_exact(&mut buf).unwrap();
        w.write(Action::SpeakerData(buf));
    }
}
