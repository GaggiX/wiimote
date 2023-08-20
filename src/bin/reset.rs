/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use wiimote_stuff::{Action, ReportingMode, Wiimote};

fn main() {
    env_logger::init_from_env("LOG");
    let w = Wiimote::open();
    w.write(Action::IRCameraEnable(None));
    w.write(Action::SpeakerMute(true));
    w.write(Action::SpeakerEnable(false));
    w.write(Action::SetReporting(ReportingMode::Buttons));
}
