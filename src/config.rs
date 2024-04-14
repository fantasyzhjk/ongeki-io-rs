use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBoardConfig {
    pub enabled: bool,
    pub test: i32,
    pub service: i32,
    pub coin: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LEDebugConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HIDConfig {
    pub enabled: bool,
    pub vid: u16,
    pub pid: u16,
    pub interface: i32,
    pub lever_left: i16,
    pub lever_right: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keyboard: KeyBoardConfig,
    pub mouse: MouseConfig,
    pub hid: HIDConfig,
    pub led_debug: LEDebugConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            keyboard: KeyBoardConfig {
                enabled: true,
                test: 0x31,
                service: 0x32,
                coin: 0x33,
            },
            mouse: MouseConfig { enabled: true },
            hid: HIDConfig {
                enabled: false,
                vid: 0x2341,
                pid: 0x8036,
                interface: 1,
                lever_left: i16::MAX,
                lever_right: i16::MIN,
            },
            led_debug: LEDebugConfig { enabled: true },
        }
    }
}
