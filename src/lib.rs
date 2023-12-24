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

/// Abstraction over the Wiimote HID
#[derive(Debug)]
pub struct Wiimote {
    device: HidDevice,
    rumble: AtomicBool,
    out: RwLock<VecDeque<Report>>,
}

impl Wiimote {
    /// Finds and opens a wiimote HID device via hidapi.
    pub fn find_hid() -> Vec<HidDevice> {
        let api = hidapi::HidApi::new().unwrap();
        let mut found = false;
        let mut product_ids = Vec::new();
        for device in api.device_list() {
            debug!("dev: {:?}", device);
            if device.vendor_id() == HID_VENDOR {
                info!("wiimote found");
                product_ids.push((device.product_id(), device.serial_number().unwrap()));
                found = true;
            }
        }
        if !found {
            return Vec::new();
        }
        let mut devices = Vec::new();
        for product_id in product_ids {
            devices.push(
                api.open_serial(HID_VENDOR, product_id.0, product_id.1)
                    .unwrap(),
            );
        }
        devices
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
    pub fn open() -> Vec<Self> {
        Self::find_hid()
            .into_iter()
            .map(Self::from_device)
            .collect()
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

/// Defines which data should be reported to the host.
#[repr(u8)]
#[derive(Debug)]
pub enum ReportingMode {
    /// Core Buttons
    Buttons = TY_CORE_BUTTONS,
    /// Core Buttons and Accelerometer
    ButtonsAccel = TY_CORE_BUTTONS_ACCELEROMETER,
    /// Core Buttons and Accelerometer with 16 Extension Bytes
    ButtonsAccelExt16 = TY_CORE_BUTTONS_ACCELEROMETER_EXTENSION16,
    /// Core Buttons and Accelerometer with 10 IR bytes and 6 Extension Bytes
    ButtonsAccelIR10Ext6 = TY_CORE_BUTTONS_ACCELEROMETER_IR10_EXTENSION6,
    /// Core Buttons and Accelerometer with 12 IR bytes
    ButtonsAccelIR12 = TY_CORE_BUTTONS_ACCELEROMETER_IR12,
    /// Core Buttons with 19 Extension bytes
    ButtonsExt19 = TY_CORE_BUTTONS_EXTENSION19,
    /// Core Buttons with 8 Extension bytes
    ButtonsExt8 = TY_CORE_BUTTONS_EXTENSION8,
    /// Core Buttons with 10 IR bytes and 9 Extension Bytes
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

/// Represents a detection by the IR camera.
#[derive(Debug, Clone, Copy)]
pub struct IRObject {
    pub x: u16,
    pub y: u16,
    /// What this exactly means depends on camera mode
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
