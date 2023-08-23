/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use wiimote_rs::{Action, IRMode, ReportingMode, IRSensitivity, Wiimote};

fn main() {
    env_logger::init_from_env("LOG");
    let w = Wiimote::open();
    let a = std::env::args().nth(1);
    if let Some(a) = a {
        match a.as_str() {
            "basic_ir" => {
                w.write(Action::IRCameraEnable(Some((
                    IRMode::Basic,
                    IRSensitivity::Level3,
                ))));
                w.write(Action::SetReporting(ReportingMode::ButtonsAccelIR10Ext6));
            }
            "accel" => {
                w.write(Action::SetReporting(ReportingMode::ButtonsAccel));
            }
            _ => panic!("unknown mode"),
        }
    }
    loop {
        eprintln!("{:?}", w.read());
    }
}
