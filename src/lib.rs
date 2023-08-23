/*
    This file is part of wiimote-rs (https://codeberg.org/metamuffin/wiimote-rs)
    which is licensed under the GNU Affero General Public License (version 3); see /COPYING.
    Copyright (C) 2023 metamuffin <metamuffin@disroot.org>
*/
mod consts;
mod read;
mod write;

use consts::*;
use hidapi::HidDevice;
use log::{debug, info};
use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, RwLock},
};

pub struct Wiimote {
    device: HidDevice,
    rumble: AtomicBool,
    out: RwLock<VecDeque<Report>>,
}

impl Wiimote {
    /// Finds and opens a wiimote HID device via hidapi.
    pub fn find_hid() -> Option<HidDevice> {
        let api = hidapi::HidApi::new().unwrap();
        let mut found = false;
        for device in api.device_list() {
            debug!("dev: {:?}", device);
            if (device.vendor_id(), device.product_id()) == (HID_VENDOR, HID_PRODUCT) {
                info!("wiimote found");
                found = true;
            }
        }
        if !found {
            return None;
        }
        let device = api.open(HID_VENDOR, HID_PRODUCT).unwrap();
        return Some(device);
    }
    /// Creates wiimote abstraction over a HidDevice (which should be a wiimote)
    pub fn from_device(device: HidDevice) -> Self {
        Self {
            device,
            rumble: false.into(),
            out: VecDeque::new().into(),
        }
    }
    /// Finds and opens a wiimote.
    pub fn open() -> Self {
        Self::from_device(Self::find_hid().expect("no wiimote found"))
    }
}

#[derive(Debug)]
pub enum Report {
    Buttons(ButtonState),
    Acceleration(Acceleration),
    LedState(u8),
    IRDetection([Option<IRObject>; 4]),
}

/// Actions control outputs like rumble, player leds, etc.
#[derive(Debug)]
pub enum Action {
    /// Sets the reporting mode, required to receive the data you need.
    SetReporting(ReportingMode),
    /// Enables the IR Camera, given mode and sensitivity. Requires 50ms to perform.
    IRCameraEnable(Option<(IRMode, IRSensitivity)>),
    /// Set player LEDs to the lower 4 bit of the provided mask.
    PlayerLeds(u8),
    /// Enables/Disables rumble
    RumbleEnable(bool),
    /// Enables/Disables speaker
    SpeakerEnable(bool),
    /// Enables/Disables speaker mute
    SpeakerMute(bool),
    /// Sends 20 samples to the speaker.
    SpeakerData([u8; 20]),
}

#[derive(Debug)]
pub enum IRSensitivity {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    CustomHigh,
    CustomMaximum,
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

#[derive(Debug, Clone, Copy)]
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum IRMode {
    Basic = 1,
    Extended = 3,
    Full = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct IRObject {
    pub x: u16,
    pub y: u16,
    pub size_or_intensity: u8,
}

#[derive(Debug, Clone, Copy, Default)]
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
