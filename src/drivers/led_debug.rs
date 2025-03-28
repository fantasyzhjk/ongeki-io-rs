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
    fn set_led_new(&mut self, board: u8, rgb: &[u8]) {
        println!("Ongeki IO: Set LED Board {board}");
        println!("rgb: {} {} {}", rgb[0], rgb[1], rgb[2]);
        println!("rgb: {} {} {}", rgb[3], rgb[4], rgb[5]);
        println!("rgb: {} {} {}", rgb[6], rgb[7], rgb[8]);
        println!("rgb: {} {} {}", rgb[9], rgb[10], rgb[11]);
        println!("rgb: {} {} {}", rgb[12], rgb[13], rgb[14]);
        println!("rgb: {} {} {}", rgb[15], rgb[16], rgb[17]);
    }
}

