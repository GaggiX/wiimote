use crate::consts::*;
use crate::{Acceleration, ButtonState, IRObject, Report, Wiimote};
use log::{trace, warn};

impl Wiimote {
    /// Read one report from the wiimote, blocking if needed
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
}

impl ButtonState {
    // Parse ButtonState from the first two bytes of the report
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

impl IRObject {
    // Parse a pair or detections from 5 bytes like in the 10-byte report.
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

    // Parse objects from 10-byte report
    pub fn from_basic(r: [u8; 10]) -> [Option<Self>; 4] {
        let [a, b] = Self::from_basic_pair([r[0], r[1], r[2], r[3], r[4]]);
        let [c, d] = Self::from_basic_pair([r[5], r[6], r[7], r[8], r[9]]);
        [a, b, c, d]
    }
}

impl Acceleration {
    /// Parse acceleration from report format
    pub fn from_report(r: &[u8]) -> Self {
        Acceleration {
            x: (r[3] as i16 - 0x80) * 2 + (r[1] & 0b01100000) as i16 >> 5,
            y: (r[4] as i16 - 0x80) * 2 + (r[2] & 0b00100000) as i16 >> 5,
            z: (r[5] as i16 - 0x80) * 2 + (r[2] & 0b01000000) as i16 >> 6,
        }
    }
}
