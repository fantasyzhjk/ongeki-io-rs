use intertrait::cast_to;

use super::{Driver, LEDriver};

#[derive(Debug, Default)]
pub struct LEDebug;

impl LEDebug {
    pub fn new() -> Self {
        Self
    }
}

#[cast_to]
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

impl Driver for LEDebug {}
