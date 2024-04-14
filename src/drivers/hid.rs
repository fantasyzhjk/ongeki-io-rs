use std::{
    io::{Cursor, Write},
    sync::mpsc::{self, Receiver},
    thread,
};

use crate::{
    config::HIDConfig,
    enums::{GameBtn, HResult},
};

use byteorder::WriteBytesExt;
use hidapi::HidDevice;
use intertrait::cast_to;
use pretty_hex::PrettyHex;

use super::{ButtonDriver, Driver, LEDriver, LeverDriver, PollDriver};

#[derive(Debug)]
pub struct HID {
    lever: i16,
    left_btns: u8,
    right_btns: u8,
    config: HIDConfig,
    device: Option<HidDevice>,
}

unsafe impl Sync for HID {}

impl HID {
    pub fn new(config: HIDConfig) -> Self {
        let mut s = HID {
            lever: 0,
            left_btns: 0,
            right_btns: 0,
            config,
            device: None,
        };
        s.try_connect_device();
        s
    }

    fn try_connect_device(&mut self) {
        if let Ok(api) = hidapi::HidApi::new() {
            self.device = api.device_list().find_map(|d| {
                if d.vendor_id() == self.config.vid
                    && d.product_id() == self.config.pid
                    && d.interface_number() == self.config.interface
                {
                    println!(
                        "Ongeki IO HID: {} 已连接",
                        d.product_string().unwrap_or_default()
                    );
                    return d.open_device(&api).ok();
                }
                None
            });
        }
    }
}

#[cast_to]
impl PollDriver for HID {
    fn poll(&mut self) -> HResult {
        let Some(ref device) = self.device else {
            self.try_connect_device();
            return HResult::Ok;
        };

        let mut data = [0u8; 64];
        if let Err(e) = device.read(&mut data) {
            println!("Ongeki IO HID: 设备断开 {e}");
            self.device = None;
            return HResult::Ok;
        }

        if data[0] == 1 {
            self.left_btns |= GameBtn::Btn1 as u8
        }
        if data[1] == 1 {
            self.left_btns |= GameBtn::Btn2 as u8
        }
        if data[2] == 1 {
            self.left_btns |= GameBtn::Btn3 as u8
        }
        if data[3] == 1 {
            self.left_btns |= GameBtn::Side as u8
        }
        if data[4] == 1 {
            self.left_btns |= GameBtn::Menu as u8
        }

        if data[5] == 1 {
            self.right_btns |= GameBtn::Btn1 as u8
        }
        if data[6] == 1 {
            self.right_btns |= GameBtn::Btn2 as u8
        }
        if data[7] == 1 {
            self.right_btns |= GameBtn::Btn3 as u8
        }
        if data[8] == 1 {
            self.right_btns |= GameBtn::Side as u8
        }
        if data[9] == 1 {
            self.right_btns |= GameBtn::Menu as u8
        }

        self.lever = i16::from_be_bytes([data[10], data[11]]);

        HResult::Ok
    }
}

#[cast_to]
impl LeverDriver for HID {
    fn lever(&self) -> i16 {

        self.lever
    }
}

#[cast_to]
impl ButtonDriver for HID {
    fn op_btns(&self) -> u8 {
        0
    }

    fn left_btns(&self) -> u8 {
        self.left_btns
    }

    fn right_btns(&self) -> u8 {
        self.right_btns
    }
}

#[cast_to]
impl LEDriver for HID {
    fn set_led(&mut self, data: u32) {
        let Some(ref device) = self.device else {
            self.try_connect_device();
            return;
        };

        let mut buf = Cursor::new([0u8; 65]);
        buf.set_position(1);
        buf.write_u8(0).unwrap();
        buf.write_u8(100).unwrap();
        buf.write_all(&[
            (((data >> 23) & 1) * 255) as u8,
            (((data >> 19) & 1) * 255) as u8,
            (((data >> 22) & 1) * 255) as u8,
            (((data >> 20) & 1) * 255) as u8,
            (((data >> 21) & 1) * 255) as u8,
            (((data >> 18) & 1) * 255) as u8,
            (((data >> 17) & 1) * 255) as u8,
            (((data >> 16) & 1) * 255) as u8,
            (((data >> 15) & 1) * 255) as u8,
            (((data >> 14) & 1) * 255) as u8,
            (((data >> 13) & 1) * 255) as u8,
            (((data >> 12) & 1) * 255) as u8,
            (((data >> 11) & 1) * 255) as u8,
            (((data >> 10) & 1) * 255) as u8,
            (((data >> 9) & 1) * 255) as u8,
            (((data >> 8) & 1) * 255) as u8,
            (((data >> 7) & 1) * 255) as u8,
            (((data >> 6) & 1) * 255) as u8,
        ])
        .unwrap();

        if let Err(e) = device.write(buf.get_ref()) {
            println!("Ongeki IO HID: 设备断开 {e}");
            self.device = None;
        }
    }
}

impl Driver for HID {}
