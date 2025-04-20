use super::{Driver, LEDriver, LEDriverNew};

use dyn_dyn::dyn_dyn_impl;

#[derive(Debug, Default)]
pub struct LEDebug;

impl LEDebug {
    pub fn new() -> Self {
        Self
    }
}

#[dyn_dyn_impl(Driver, LEDriver, LEDriverNew)]
impl Driver for LEDebug {}

impl LEDriver for LEDebug {
    fn set_led(&mut self, data: u32) {
        println!(
            "Ongeki IO: Set LED\n{} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {}, {} {} {}",
            ((data >> 23) & 1) * 255,
            ((data >> 19) & 1) * 255,
            ((data >> 22) & 1) * 255,
            ((data >> 20) & 1) * 255,
            ((data >> 21) & 1) * 255,
            ((data >> 18) & 1) * 255,
            ((data >> 17) & 1) * 255,
            ((data >> 16) & 1) * 255,
            ((data >> 15) & 1) * 255,
            ((data >> 14) & 1) * 255,
            ((data >> 13) & 1) * 255,
            ((data >> 12) & 1) * 255,
            ((data >> 11) & 1) * 255,
            ((data >> 10) & 1) * 255,
            ((data >> 9) & 1) * 255,
            ((data >> 8) & 1) * 255,
            ((data >> 7) & 1) * 255,
            ((data >> 6) & 1) * 255
        );
    }
}


impl LEDriverNew for LEDebug {
    fn set_led_new(&mut self, board: u8, rgb: &[rgb::RGB8]) {
        println!("Ongeki IO: Set LED Board {board}");
        for rgb in rgb {
            print!("{:X} ", rgb);
        }
        println!();
    }
}

