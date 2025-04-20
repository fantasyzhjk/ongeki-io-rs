use std::io::{Cursor, Write};

use crate::{
    config::HIDConfig,
    enums::{GameBtn, HResult},
};

use super::{ButtonDriver, Driver, LEDriver, LeverDriver, PollDriver, LEDriverNew};

use byteorder::WriteBytesExt;
use dyn_dyn::dyn_dyn_impl;
use hidapi::{HidApi, HidDevice};

pub struct HidIO {
    lever: i16,
    left_btns: u8,
    right_btns: u8,
    config: HIDConfig,
    device: Option<HidDevice>,
}

#[dyn_dyn_impl(Driver, PollDriver, ButtonDriver, LeverDriver, LEDriver, LEDriverNew)]
impl Driver for HidIO {}
unsafe impl Sync for HidIO {}

impl HidIO {
    pub fn new(config: HIDConfig) -> Self {
        let mut s = HidIO {
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
        let api = HidApi::new().unwrap();
        self.device = api
            .device_list()
            .filter(|d| d.vendor_id() == self.config.vid && d.product_id() == self.config.pid)
            .find_map(|d| {
                if d.interface_number() == self.config.interface {
                    println!(
                        "Ongeki IO HID: {} 已连接",
                        d.product_string().unwrap_or_default()
                    );
                    return d.open_device(&api).inspect(|d| {
                        d.set_blocking_mode(false).unwrap();
                    }).ok();
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

        // self.lever = -20 * i16::from_be_bytes([data[10], data[11]]);
        // Auto Calculation
        let lever_meta = i16::from_be_bytes([data[10], data[11]]);
        if self.config.lever_left > self.config.lever_right {
            if lever_meta > self.config.lever_left {
                self.config.lever_left = lever_meta;
            }
            if lever_meta < self.config.lever_right {
                self.config.lever_right = lever_meta;
            }
        } else {
            if lever_meta < self.config.lever_left {
                self.config.lever_left = lever_meta;
            }
            if lever_meta > self.config.lever_right {
                self.config.lever_right = lever_meta;
            }
        }

        // 映射前动态交换左右边界，确保方向正确
        let (in_min, in_max) = if self.config.lever_left < self.config.lever_right {
            (self.config.lever_left, self.config.lever_right)
        } else {
            // 如果校准值方向颠倒，交换它们
            (self.config.lever_right, self.config.lever_left)
        };

        if self.config.lever_right != self.config.lever_left {
            self.lever = map(
                i32::from(lever_meta),
                i32::from(in_min),
                i32::from(in_max),
                -32768,
                32768,
            ) as i16;
        }

        HResult::Ok
    }
}

pub(crate) fn map(x: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    // 自动处理反向输入（例如 in_min > in_max）
    let numerator = (x - in_min) * (out_max - out_min);
    let denominator = in_max - in_min;
    if denominator == 0 {
        return out_min; // 避免除以零
    }
    numerator / denominator + out_min
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


impl LEDriverNew for HidIO {
    fn set_led_new(&mut self, board: u8, rgb: &[rgb::RGB8]) {
        if board == 1 {
            let Some(ref device) = self.device else {
                self.try_connect_device();
                return;
            };
    
            let mut buf = Cursor::new([0u8; 65]);
            buf.set_position(1);
            buf.write_u8(0).unwrap();
            buf.write_u8(100).unwrap();
            buf.write_all(
                &[
                    rgb[0].r, rgb[0].g, rgb[0].b,
                    rgb[1].r, rgb[1].g, rgb[1].b,
                    rgb[2].r, rgb[2].g, rgb[2].b,
                    rgb[3].r, rgb[3].g, rgb[3].b,
                    rgb[4].r, rgb[4].g, rgb[4].b,
                    rgb[5].r, rgb[5].g, rgb[5].b
                ],
            ).unwrap();

            if let Err(e) = device.write(buf.get_ref()) {
                println!("Ongeki IO HID: 设备断开 {e}");
                self.device = None;
            }
        }
    }
}


/// test
#[cfg(test)]
mod hid_test {
    use super::map;
    #[test]
    fn map_test() {
        let temp = map(12, -280, 280, -20000, 20000);
        assert_eq!(temp, 857);
    }
}