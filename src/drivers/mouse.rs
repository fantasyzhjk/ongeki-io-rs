use dyn_dyn::dyn_dyn_impl;
use windows::Win32::UI::WindowsAndMessaging::{self, SM_CXSCREEN};
use windows::Win32::Foundation::POINT;

use crate::drivers::{Driver, LeverDriver, PollDriver};
use crate::enums::HResult;


#[derive(Debug, Default)]
pub struct MouseIO {
    lever: i16,
}

impl MouseIO {
    pub fn new() -> Self {
        Self::default()
    }
}

#[dyn_dyn_impl(Driver, PollDriver, LeverDriver)]
impl Driver for MouseIO {}

impl PollDriver for MouseIO {
    fn poll(&mut self) -> HResult {
        unsafe {
            let mut p = POINT::default();
            WindowsAndMessaging::GetCursorPos(&mut p as *mut POINT).unwrap();
            let mut mouse_x = p.x;
            let screen_width = WindowsAndMessaging::GetSystemMetrics(SM_CXSCREEN);
            if mouse_x < 0 {
                mouse_x = 0;
            } else if mouse_x > screen_width {
                mouse_x = screen_width;
            }

            let x_norm = mouse_x as f64 / screen_width as f64;
            let mouse_x = ((x_norm * 65536.) - 32767.) as i32;

            self.lever = mouse_x as i16;
        }
        HResult::Ok
    }
}

impl LeverDriver for MouseIO {
    fn lever(&self) -> i16 {
        self.lever
    }
}

