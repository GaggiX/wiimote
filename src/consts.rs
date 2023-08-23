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
