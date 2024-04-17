use crate::{config::KeyBoardConfig, enums::{GameBtn, HResult, OpBtn}};
use super::{ButtonDriver, Driver, PollDriver};

use dyn_dyn::dyn_dyn_impl;
use windows::Win32::UI::Input::KeyboardAndMouse;


#[derive(Debug)]
pub struct KeyBoardIO {
    op_btns: u8,
    left_btns: u8,
    right_btns: u8,
    config: KeyBoardConfig
}

impl KeyBoardIO {
    pub fn new(config: KeyBoardConfig) -> Self {
        Self {
            op_btns: 0,
            left_btns: 0,
            right_btns: 0,
            config,
        }
    }
}

#[dyn_dyn_impl(Driver, PollDriver, ButtonDriver)]
impl Driver for KeyBoardIO {}

impl PollDriver for KeyBoardIO {
    fn poll(&mut self) -> HResult {
        self.op_btns = 0;

        if is_key_pressed(self.config.test) {
            self.op_btns |= OpBtn::Test as u8
        }
        if is_key_pressed(self.config.service) {
            self.op_btns |= OpBtn::Service as u8
        }
        if is_key_pressed(self.config.coin) {
            self.op_btns |= OpBtn::Coin as u8
        }

        self.left_btns = 0;
        self.right_btns = 0;

        if is_key_pressed(0x41) {
            self.left_btns |= GameBtn::Btn1 as u8
        }
        if is_key_pressed(0x53) {
            self.left_btns |= GameBtn::Btn2 as u8
        }
        if is_key_pressed(0x44) {
            self.left_btns |= GameBtn::Btn3 as u8
        }
        if is_key_pressed(0x4A) {
            self.right_btns |= GameBtn::Btn1 as u8
        }
        if is_key_pressed(0x4B) {
            self.right_btns |= GameBtn::Btn2 as u8
        }
        if is_key_pressed(0x4C) {
            self.right_btns |= GameBtn::Btn3 as u8
        }
        if is_key_pressed(0x55) {
            self.left_btns |= GameBtn::Menu as u8
        }
        if is_key_pressed(0x4F) {
            self.right_btns |= GameBtn::Menu as u8
        }
        if is_key_pressed(0x01) {
            self.left_btns |= GameBtn::Side as u8
        }
        if is_key_pressed(0x02) {
            self.right_btns |= GameBtn::Side as u8
        }

        HResult::Ok
    }
}

impl ButtonDriver for KeyBoardIO {
    fn op_btns(&self) -> u8 {
        self.op_btns
    }

    fn left_btns(&self) -> u8 {
        self.left_btns
    }

    fn right_btns(&self) -> u8 {
        self.right_btns
    }
}


fn is_key_pressed(key: i32) -> bool {
    unsafe { KeyboardAndMouse::GetAsyncKeyState(key) != 0 }
}
