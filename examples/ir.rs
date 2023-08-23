/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use wiimote_rs::{Action, IRMode, Report, ReportingMode, Sensitivity, Wiimote};

fn main() {
    env_logger::init_from_env("LOG");
    let w = Wiimote::open();
    w.write(Action::IRCameraEnable(None));
    w.write(Action::IRCameraEnable(Some((
        IRMode::Basic,
        Sensitivity::Level3,
    ))));
    w.write(Action::SetReporting(ReportingMode::ButtonsIR10Ext9));
    loop {
        if let Some(ev) = w.read() {
            match ev {
                Report::IRDetection(objects) => {
                    eprintln!("{objects:#?}")
                }
                _ => (),
            }
        }
    }
}
