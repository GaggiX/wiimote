use crate::{consts::*, Action, IRSensitivity, Wiimote};
use log::trace;
use std::{sync::atomic::Ordering, thread::sleep, time::Duration};

impl Wiimote {
    /// Sends a packet to the wiimote and makes sure that we dont influence rumble :)
    fn write_inner(&self, bytes: &mut [u8]) {
        if self.rumble.load(Ordering::Relaxed) {
            bytes[1] = bytes[1] | 1;
        } else {
            bytes[1] = bytes[1] & !1;
        }
        trace!("send {bytes:02x?}");
        self.device.write(bytes).unwrap();
    }

    /// Writes data to wiimotes internal registers or memory.
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

    /// Enable/Disable a feature
    fn set_enabled(&self, feature: u8, enable: bool) {
        self.write_inner(&mut [
            feature,
            if enable {
                VA_FEATURE_ENABLE
            } else {
                VA_FEATURE_DISABLE
            },
        ])
    }

    /// Send an action to the wiimote. (Enabling the camera takes 50ms)
    pub fn write(&self, a: Action) {
        match a {
            Action::RumbleEnable(enable) => {
                self.rumble.store(enable, Ordering::Relaxed);
                self.write_inner(&mut [TY_RUMBLE, enable as u8])
            }
            Action::SpeakerEnable(enable) => self.set_enabled(TY_SPEAKER_ENABLE, enable),
            Action::SpeakerMute(enable) => self.set_enabled(TY_SPEAKER_MUTE, enable),
            Action::IRCameraEnable(enable) => {
                if let Some((mode, sens)) = enable {
                    let sens = sens.blocks();
                    self.set_enabled(TY_IR_CAMERA_PIXEL_CLOCK_ENABLE, true);
                    self.set_enabled(TY_IR_CAMERA_CHIP_ENABLE, true);
                    self.write_registers(REG_IR, &[0x01]);
                    sleep(Duration::from_millis(50)); // wiibrew wiki says this might help...
                    self.write_registers(REG_IR_SENS_BLOCK1, &sens.0);
                    self.write_registers(REG_IR_SENS_BLOCK2, &sens.1);
                    self.write_registers(REG_IR_MODE, &[mode as u8]);
                    self.write_registers(REG_IR, &[0x08]);
                } else {
                    self.set_enabled(TY_IR_CAMERA_CHIP_ENABLE, false);
                    self.set_enabled(TY_IR_CAMERA_PIXEL_CLOCK_ENABLE, false);
                }
            }
            Action::PlayerLeds(mask) => self.write_inner(&mut [TY_PLAYER_LEDS, mask << 4]),
            Action::SpeakerData(data) => {
                let mut to_send = [0; 22];
                to_send[0] = TY_SPEAKER_DATA;
                to_send[1] = 20 << 3;
                to_send[2..].copy_from_slice(&data);
                self.write_inner(&mut to_send);
                sleep(Duration::from_millis(10));
            }
            Action::SetReporting(r) => self.write_inner(&mut [TY_SET_REPORTING, 0x00, r as u8]),
        }
    }
}

impl IRSensitivity {
    pub fn blocks(&self) -> ([u8; 9], [u8; 2]) {
        match self {
            IRSensitivity::Level1 => IR_SENS_LEVEL1,
            IRSensitivity::Level2 => IR_SENS_LEVEL2,
            IRSensitivity::Level3 => IR_SENS_LEVEL3,
            IRSensitivity::Level4 => IR_SENS_LEVEL4,
            IRSensitivity::Level5 => IR_SENS_LEVEL5,
            IRSensitivity::CustomHigh => IR_SENS_CUSTOM_HIGH,
            IRSensitivity::CustomMaximum => IR_SENS_CUSTOM_MAX,
        }
    }
}
