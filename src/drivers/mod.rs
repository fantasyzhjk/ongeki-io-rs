use intertrait::cast::*;
use intertrait::*;
use std::fs::{self, File};
use std::io::Write;
use std::ops::{Deref, DerefMut};

use crate::config::Config;
use crate::drivers::hid::HID;
use crate::enums::HResult;

trait Driver: CastFrom {}

mod keyboard;
mod led_debug;
mod mouse;
mod hid;

use self::keyboard::KeyBoard;
use self::led_debug::LEDebug;
use self::mouse::Mouse;

trait PollDriver {
    fn poll(&mut self) -> HResult;
}

trait ButtonDriver {
    fn op_btns(&self) -> u8;
    fn left_btns(&self) -> u8;
    fn right_btns(&self) -> u8;
}

trait LeverDriver {
    fn lever(&self) -> i16;
}

trait LEDriver {
    fn set_led(&mut self, data: u32);
}

pub struct Drivers(Vec<Box<dyn Driver + Sync + Send>>);

impl Drivers {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn init(&mut self) {
        const CONFIG_PATH: &str = "ongeki-io.toml";
        let mut config = Config::default();

        if let Ok(s) = fs::read_to_string(CONFIG_PATH) {
            config = toml::from_str(&s).unwrap();
            println!("Ongeki IO: 使用配置文件\n{:#?}", config);
        } else {
            let mut f = File::create(CONFIG_PATH).unwrap();
            f.write_all(toml::to_string_pretty(&config).unwrap().as_bytes()).unwrap();
            println!("Ongeki IO: 未发现配置文件，使用默认配置\n{:#?}", config);
        }

        if config.keyboard.enabled {
            self.0.push(Box::new(KeyBoard::new(config.keyboard.clone())));
        }
        if config.mouse.enabled {
            self.0.push(Box::new(Mouse::new()));
        }
        if config.led_debug.enabled {
            self.0.push(Box::new(LEDebug::new()));
        }
        if config.hid.enabled {
            self.0.push(Box::new(HID::new(config.hid.clone())));
        }
    }

    pub fn poll(&mut self) {
        for driver in self.0.iter_mut() {
            if let Some(d) = driver.deref_mut().cast::<dyn PollDriver>() {
                d.poll();
            }
        }
    }

    pub fn op_btns(&self) -> u8 {
        self.0
            .iter()
            .filter_map(|d| d.deref().cast::<dyn ButtonDriver>())
            .map(|d| d.op_btns())
            .fold(0, |r, v| r | v)
    }

    pub fn left_btns(&self) -> u8 {
        self.0
            .iter()
            .filter_map(|d| d.deref().cast::<dyn ButtonDriver>())
            .map(|d| d.left_btns())
            .fold(0, |r, v| r | v)
    }

    pub fn right_btns(&self) -> u8 {
        self.0
            .iter()
            .filter_map(|d| d.deref().cast::<dyn ButtonDriver>())
            .map(|d| d.right_btns())
            .fold(0, |r, v| r | v)
    }

    pub fn lever(&self) -> Option<i16> {
        self.0
            .iter()
            .filter_map(|d| d.deref().cast::<dyn LeverDriver>())
            .map(|d| d.lever())
            .next()
    }

    pub fn set_led(&mut self, data: u32) {
        for driver in self.0.iter_mut() {
            if let Some(d) = driver.deref_mut().cast::<dyn LEDriver>() {
                d.set_led(data);
            }
        }
    }
}
