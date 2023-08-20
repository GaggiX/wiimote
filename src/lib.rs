/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use hidapi::HidDevice;
use log::{debug, info, trace};
use std::{thread::sleep, time::Duration};

pub struct Wiimote {
    device: HidDevice,
    rumble: bool,
}

impl Wiimote {
    pub fn open() -> Self {
        let api = hidapi::HidApi::new().unwrap();
        let mut found = false;
        for device in api.device_list() {
            debug!("dev: {:?}", device);
            if (device.vendor_id(), device.product_id()) == (0x057e, 0x0306) {
                info!("wiimote found");
                found = true;
            }
        }
        if !found {
            panic!("no wiimote found");
        }
        let device = api.open(0x057e, 0x0306).unwrap();
        Self {
            device,
            rumble: false,
        }
    }

    pub fn read(&self) {
        let mut buf = [0u8; 1024];
        let size = self.device.read(&mut buf).unwrap();
        debug!("recv {:?}", &buf[..size]);
    }

    fn write_inner(&self, bytes: &mut [u8]) {
        if self.rumble {
            bytes[1] = bytes[1] | 1;
        } else {
            bytes[1] = bytes[1] & !1;
        }
        trace!("send {bytes:02x?}");
        self.device.write(bytes).unwrap();
    }
    pub fn write_registers(&self, addr: u32, data: &[u8]) {
        let mut bytes = [0; 22];
        bytes[0] = consts::WRITE_MEMORY_AND_REGISTERS;
        bytes[1] = 0x04;
        bytes[2..5].copy_from_slice(&addr.to_be_bytes()[1..]);
        let data_len = 16.min(data.len());
        bytes[5] = data_len as u8;
        bytes[6..6 + data_len].copy_from_slice(&data[0..data_len]);
        let ret = self.write_inner(&mut bytes);
        sleep(Duration::from_millis(10));
        ret
    }

    fn set_enabled(&self, feature: u8, enable: bool) {
        self.write_inner(&mut [feature, if enable { 0x04 } else { 0x00 }])
    }

    pub fn write(&self, a: Action) {
        match a {
            Action::SpeakerEnable(enable) => self.set_enabled(consts::SPEAKER_ENABLE, enable),
            Action::SpeakerMute(enable) => self.set_enabled(consts::SPEAKER_MUTE, enable),
            Action::SpeakerData(data) => {
                let mut to_send = [0; 22];
                to_send[0] = consts::SPEAKER_DATA;
                to_send[1] = 20 << 3;
                to_send[2..].copy_from_slice(&data);
                self.write_inner(&mut to_send);
                sleep(Duration::from_millis(10));
            }
        }
    }
}

pub enum Report {}
pub enum Action {
    SpeakerEnable(bool),
    SpeakerMute(bool),
    SpeakerData([u8; 20]),
}

mod consts {
    pub const RUMBLE: u8 = 0x10;
    pub const PLAYER_LEDS: u8 = 0x11;
    pub const DATA_REPORTING_MODE: u8 = 0x12;
    pub const IR_CAMERA_ENABLE: u8 = 0x13;
    pub const SPEAKER_ENABLE: u8 = 0x14;
    pub const STATUS_INFORMATION_REQUEST: u8 = 0x15;
    pub const WRITE_MEMORY_AND_REGISTERS: u8 = 0x16;
    pub const READ_MEMORY_AND_REGISTERS: u8 = 0x17;
    pub const SPEAKER_DATA: u8 = 0x18;
    pub const SPEAKER_MUTE: u8 = 0x19;
    pub const IR_CAMERA_ENABLE2: u8 = 0x1a;
    pub const STATUS_INFORMATION: u8 = 0x1b;
    pub const READ_MEMORY_AND_REGISTERS_DATA: u8 = 0x1c;
    pub const RESULT: u8 = 0x1d;
    pub const CORE_BUTTONS: u8 = 0x30;
    pub const CORE_BUTTONS_ACCELEROMETER: u8 = 0x31;
    pub const CORE_BUTTONS_EXTENSION8: u8 = 0x32;
    pub const CORE_BUTTONS_ACCELEROMETER_IR12: u8 = 0x33;
    pub const CORE_BUTTONS_EXTENSION19: u8 = 0x34;
    pub const CORE_BUTTONS_ACCELEROMETER_EXTENSION16: u8 = 0x35;
    pub const CORE_BUTTONS_IR10_EXTENSION9: u8 = 0x36;
    pub const CORE_BUTTONS_ACCELEROMETER_IR10_EXTENSION6: u8 = 0x37;
    pub const EXTENSION21: u8 = 0x3d;
    pub const INTERLEAVED0: u8 = 0x3e;
    pub const INTERLEAVED1: u8 = 0x3f;
}
