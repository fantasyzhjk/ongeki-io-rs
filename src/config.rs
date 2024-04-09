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
pub struct Config {
    pub keyboard: KeyBoardConfig,
    pub mouse: MouseConfig,
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
            led_debug: LEDebugConfig { enabled: true },
        }
    }
}
