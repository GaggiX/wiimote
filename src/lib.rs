/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
use consts::*;
use hidapi::HidDevice;
use log::{debug, info, trace, warn};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        RwLock,
    },
    thread::sleep,
    time::Duration,
};

pub struct Wiimote {
    device: HidDevice,
    rumble: AtomicBool,
    out: RwLock<VecDeque<Report>>,
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
            rumble: false.into(),
            out: VecDeque::new().into(),
        }
    }

    pub fn read(&self) -> Option<Report> {
        let mut out = self.out.write().unwrap();
        if !out.is_empty() {
            return out.pop_front();
        }
        let mut buf = [0u8; 1024];
        let size = self.device.read(&mut buf).unwrap();
        trace!("recv {:?}", &buf[..size]);
        match buf[0] {
            TY_CORE_BUTTONS_IR10_EXTENSION9 => {
                out.push_back(Report::Buttons(ButtonState::from_flags([buf[1], buf[2]])));
                out.push_back(Report::IRDetection(IRObject::from_basic(
                    buf[3..13].try_into().unwrap(),
                )))
            }
            TY_CORE_BUTTONS_ACCELEROMETER_IR10_EXTENSION6 => {
                out.push_back(Report::Buttons(ButtonState::from_flags([buf[1], buf[2]])));
                out.push_back(Report::Acceleration(Acceleration::from_report(&buf)));
                out.push_back(Report::IRDetection(IRObject::from_basic(
                    buf[6..16].try_into().unwrap(),
                )))
            }
            TY_CORE_BUTTONS_ACCELEROMETER => {
                out.push_back(Report::Buttons(ButtonState::from_flags([buf[1], buf[2]])));
                out.push_back(Report::Acceleration(Acceleration::from_report(&buf)))
            }
            TY_CORE_BUTTONS => {
                out.push_back(Report::Buttons(ButtonState::from_flags([buf[1], buf[2]])));
            }
            TY_STATUS_INFORMATION => {
                out.push_back(Report::LedState(buf[3] >> 4));
            }
            TY_READ_MEMORY_AND_REGISTERS_DATA => {}
            TY_RESULT => {}
            x => {
                warn!("unknown report type: {x:02x}");
            }
        }
        out.pop_front()
    }

    fn write_inner(&self, bytes: &mut [u8]) {
        if self.rumble.load(Ordering::Relaxed) {
            bytes[1] = bytes[1] | 1;
        } else {
            bytes[1] = bytes[1] & !1;
        }
        trace!("send {bytes:02x?}");
        self.device.write(bytes).unwrap();
    }

    pub fn write_registers(&self, addr: u32, data: &[u8]) {
        let mut bytes = [0; 22];
        bytes[0] = TY_WRITE_MEMORY_AND_REGISTERS;
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
            Action::RumbleEnable(enable) => {
                self.rumble.store(enable, Ordering::Relaxed);
                self.write_inner(&mut [0x10, enable as u8])
            }
            Action::SpeakerEnable(enable) => self.set_enabled(TY_SPEAKER_ENABLE, enable),
            Action::SpeakerMute(enable) => self.set_enabled(TY_SPEAKER_MUTE, enable),
            Action::IRCameraEnable(enable) => {
                if let Some(mode) = enable {
                    let sens = IR_SENS_LEVEL3;
                    self.set_enabled(TY_IR_CAMERA_PIXEL_CLOCK_ENABLE, true);
                    self.set_enabled(TY_IR_CAMERA_CHIP_ENABLE, true);
                    self.write_registers(REG_IR, &[0x01]);
                    sleep(Duration::from_millis(50));
                    self.write_registers(REG_IR_SENS_BLOCK1, &sens.0);
                    self.write_registers(REG_IR_SENS_BLOCK2, &sens.1);
                    self.write_registers(REG_IR_MODE, &[mode as u8]);
                    self.write_registers(REG_IR, &[0x80]);
                } else {
                    self.set_enabled(TY_IR_CAMERA_CHIP_ENABLE, false);
                    self.set_enabled(TY_IR_CAMERA_PIXEL_CLOCK_ENABLE, false);
                }
            }
            Action::PlayerLeds(mask) => self.write_inner(&mut [0x11, mask << 4]),
            Action::SpeakerData(data) => {
                let mut to_send = [0; 22];
                to_send[0] = TY_SPEAKER_DATA;
                to_send[1] = 20 << 3;
                to_send[2..].copy_from_slice(&data);
                self.write_inner(&mut to_send);
                sleep(Duration::from_millis(10));
            }
            Action::SetReporting(r) => self.write_inner(&mut [0x12, 0x00, r as u8]),
        }
    }
}

#[derive(Debug)]
pub enum Report {
    Buttons(ButtonState),
    Acceleration(Acceleration),
    LedState(u8),
    IRDetection([Option<IRObject>; 4]),
}
#[derive(Debug)]
pub enum Action {
    SetReporting(ReportingMode),
    IRCameraEnable(Option<IRMode>),
    PlayerLeds(u8),
    RumbleEnable(bool),
    SpeakerEnable(bool),
    SpeakerMute(bool),
    SpeakerData([u8; 20]),
}

#[repr(u8)]
#[derive(Debug)]
pub enum ReportingMode {
    Buttons = TY_CORE_BUTTONS,
    ButtonsAccel = TY_CORE_BUTTONS_ACCELEROMETER,
    ButtonsAccelExt16 = TY_CORE_BUTTONS_ACCELEROMETER_EXTENSION16,
    ButtonsAccelIR10Ext6 = TY_CORE_BUTTONS_ACCELEROMETER_IR10_EXTENSION6,
    ButtonsAccelIR12 = TY_CORE_BUTTONS_ACCELEROMETER_IR12,
    ButtonsExt19 = TY_CORE_BUTTONS_EXTENSION19,
    ButtonsExt8 = TY_CORE_BUTTONS_EXTENSION8,
    ButtonsIR10Ext9 = TY_CORE_BUTTONS_IR10_EXTENSION9,
}

#[derive(Debug)]
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl Acceleration {
    pub fn from_report(r: &[u8]) -> Self {
        Acceleration {
            x: (r[3] as i16 - 0x80) * 2 + (r[1] & 0b01100000) as i16 >> 5,
            y: (r[4] as i16 - 0x80) * 2 + (r[2] & 0b00100000) as i16 >> 5,
            z: (r[5] as i16 - 0x80) * 2 + (r[2] & 0b01000000) as i16 >> 6,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum IRMode {
    Basic = 1,
    Extended = 3,
    Full = 5,
}

#[derive(Debug)]
pub struct IRObject {
    pub x: u16,
    pub y: u16,
    pub size_or_intensity: u8,
}
impl IRObject {
    fn from_basic_pair(r: [u8; 5]) -> [Option<Self>; 2] {
        [
            if r[0] == 0xff && r[1] == 0xff {
                None
            } else {
                Some(Self {
                    size_or_intensity: 0,
                    x: r[0] as u16 | ((r[2] & 0b00110000) as u16) << 4,
                    y: r[1] as u16 | ((r[2] & 0b11000000) as u16) << 2,
                })
            },
            if r[3] == 0xff && r[4] == 0xff {
                None
            } else {
                Some(Self {
                    size_or_intensity: 0,
                    x: r[3] as u16 | ((r[2] & 0b00000011) as u16) << 8,
                    y: r[4] as u16 | ((r[2] & 0b00001100) as u16) << 6,
                })
            },
        ]
    }
    pub fn from_basic(r: [u8; 10]) -> [Option<Self>; 4] {
        let [a, b] = Self::from_basic_pair([r[0], r[1], r[2], r[3], r[4]]);
        let [c, d] = Self::from_basic_pair([r[5], r[6], r[7], r[8], r[9]]);
        [a, b, c, d]
    }
}

#[derive(Debug)]
pub struct ButtonState {
    pub d_pad_left: bool,
    pub d_pad_right: bool,
    pub d_pad_down: bool,
    pub d_pad_up: bool,
    pub plus: bool,
    pub two: bool,
    pub one: bool,
    pub b: bool,
    pub a: bool,
    pub minus: bool,
    pub home: bool,
}
impl ButtonState {
    #[rustfmt::skip]
    pub fn from_flags([x, y]: [u8; 2]) -> Self {
        Self {
            d_pad_left:  x & 0b00000001 != 0,
            d_pad_right: x & 0b00000010 != 0,
            d_pad_down:  x & 0b00000100 != 0,
            d_pad_up:    x & 0b00001000 != 0,
            plus:        x & 0b00010000 != 0,
            two:         y & 0b00000001 != 0,
            one:         y & 0b00000010 != 0,
            b:           y & 0b00000100 != 0,
            a:           y & 0b00001000 != 0,
            minus:       y & 0b00010000 != 0,
            home:        y & 0b10000000 != 0,
        }
    }
}

pub mod consts {
    pub const REG_IR: u32 = 0xb00030;
    pub const REG_IR_SENS_BLOCK1: u32 = 0xb00000;
    pub const REG_IR_SENS_BLOCK2: u32 = 0xb0001a;
    pub const REG_IR_MODE: u32 = 0xb00033;

    pub const IR_SENS_LEVEL1: ([u8; 9], [u8; 2]) =
        (*b"\x02\x00\x00\x71\x01\x00\x64\x00\xfe", *b"\xfd\x05");
    pub const IR_SENS_LEVEL2: ([u8; 9], [u8; 2]) =
        (*b"\x02\x00\x00\x71\x01\x00\x96\x00\xb4", *b"\xb3\x04");
    pub const IR_SENS_LEVEL3: ([u8; 9], [u8; 2]) =
        (*b"\x02\x00\x00\x71\x01\x00\xaa\x00\x64", *b"\x63\x03");
    pub const IR_SENS_LEVEL4: ([u8; 9], [u8; 2]) =
        (*b"\x02\x00\x00\x71\x01\x00\xc8\x00\x36", *b"\x35\x03");
    pub const IR_SENS_LEVEL5: ([u8; 9], [u8; 2]) =
        (*b"\x07\x00\x00\x71\x01\x00\x72\x00\x20", *b"\x1f\x03");

    pub const TY_RUMBLE: u8 = 0x10;
    pub const TY_PLAYER_LEDS: u8 = 0x11;
    pub const TY_DATA_REPORTING_MODE: u8 = 0x12;
    pub const TY_IR_CAMERA_PIXEL_CLOCK_ENABLE: u8 = 0x13;
    pub const TY_SPEAKER_ENABLE: u8 = 0x14;
    pub const TY_STATUS_INFORMATION_REQUEST: u8 = 0x15;
    pub const TY_WRITE_MEMORY_AND_REGISTERS: u8 = 0x16;
    pub const TY_READ_MEMORY_AND_REGISTERS: u8 = 0x17;
    pub const TY_SPEAKER_DATA: u8 = 0x18;
    pub const TY_SPEAKER_MUTE: u8 = 0x19;
    pub const TY_IR_CAMERA_CHIP_ENABLE: u8 = 0x1a;
    pub const TY_STATUS_INFORMATION: u8 = 0x20;
    pub const TY_READ_MEMORY_AND_REGISTERS_DATA: u8 = 0x21;
    pub const TY_RESULT: u8 = 0x22;
    pub const TY_CORE_BUTTONS: u8 = 0x30;
    pub const TY_CORE_BUTTONS_ACCELEROMETER: u8 = 0x31;
    pub const TY_CORE_BUTTONS_EXTENSION8: u8 = 0x32;
    pub const TY_CORE_BUTTONS_ACCELEROMETER_IR12: u8 = 0x33;
    pub const TY_CORE_BUTTONS_EXTENSION19: u8 = 0x34;
    pub const TY_CORE_BUTTONS_ACCELEROMETER_EXTENSION16: u8 = 0x35;
    pub const TY_CORE_BUTTONS_IR10_EXTENSION9: u8 = 0x36;
    pub const TY_CORE_BUTTONS_ACCELEROMETER_IR10_EXTENSION6: u8 = 0x37;
    pub const TY_EXTENSION21: u8 = 0x3d;
    pub const TY_INTERLEAVED0: u8 = 0x3e;
    pub const TY_INTERLEAVED1: u8 = 0x3f;
}
