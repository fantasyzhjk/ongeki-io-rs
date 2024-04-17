use std::io::{Cursor, Write};

use crate::{
    config::HIDConfig,
    enums::{GameBtn, HResult},
};

use super::{ButtonDriver, Driver, LEDriver, LeverDriver, PollDriver};

use byteorder::WriteBytesExt;
use dyn_dyn::dyn_dyn_impl;
use hidapi_rusb::{HidApi, HidDevice};

pub struct HidIO {
    lever: i16,
    left_btns: u8,
    right_btns: u8,
    config: HIDConfig,
    api: HidApi,
    device: Option<HidDevice>,
}

#[dyn_dyn_impl(Driver, PollDriver, ButtonDriver, LeverDriver, LEDriver)]
impl Driver for HidIO {}
unsafe impl Sync for HidIO {}

impl HidIO {
    pub fn new(config: HIDConfig) -> Self {
        let mut s = HidIO {
            lever: 0,
            left_btns: 0,
            right_btns: 0,
            config,
            api: HidApi::new().unwrap(),
            device: None,
        };
        s.try_connect_device();
        s
    }

    fn try_connect_device(&mut self) {
        self.api.refresh_devices().unwrap();
        self.device = self.api.device_list().find_map(|d| {
            if d.vendor_id() == self.config.vid
                && d.product_id() == self.config.pid
                && d.interface_number() == self.config.interface
            {
                println!(
                    "Ongeki IO HID: {} 已连接",
                    d.product_string().unwrap_or_default()
                );
                return d.open_device(&self.api).ok();
            }
            None
        });
    }
}

impl PollDriver for HidIO {
    fn poll(&mut self) -> HResult {
        let Some(ref device) = self.device else {
            self.try_connect_device();
            return HResult::Ok;
        };

        let mut data = [0u8; 64];
        self.left_btns = 0;
        self.right_btns = 0;
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

        self.lever = -20 * i16::from_be_bytes([data[10], data[11]]);
        // Auto Calculation
        // let lever_meta = i16::from_be_bytes([data[10], data[11]]);
        // let lever_dir: i8 = if self.config.lever_left < self.config.lever_right {
        //     1
        // } else {
        //     -1
        // };
        // if lever_dir == -1 {
        //     if lever_meta < self.config.lever_left {
        //         self.config.lever_left = lever_meta;
        //     }
        //     if lever_meta > self.config.lever_right {
        //         self.config.lever_right = lever_meta;
        //     }
        // }
        // if lever_dir == 1 {
        //     if lever_meta > self.config.lever_left {
        //         self.config.lever_left = lever_meta;
        //     }
        //     if lever_meta < self.config.lever_right {
        //         self.config.lever_right = lever_meta;
        //     }
        // }

        // if self.config.lever_right != self.config.lever_left
        // {
        //     self.lever = map(lever_meta, self.config.lever_left, self.config.lever_right, -20000, 20000);
        // }

        HResult::Ok
    }
}

fn map(x: i16, in_min: i16, in_max: i16, out_min: i16, out_max: i16) -> i16 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}


impl LeverDriver for HidIO {
    fn lever(&self) -> i16 {
        self.lever
    }
}

impl ButtonDriver for HidIO {
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

impl LEDriver for HidIO {
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
