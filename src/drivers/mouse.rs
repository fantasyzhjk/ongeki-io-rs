use crate::enums::HResult;
use intertrait::cast_to;
use windows::Win32::UI::WindowsAndMessaging::{self, SM_CXSCREEN};
use windows::Win32::Foundation::POINT;

use super::{Driver, LeverDriver, PollDriver};

#[derive(Debug, Default)]
pub struct Mouse {
    lever: i16,
}

impl Mouse {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cast_to]
impl PollDriver for Mouse {
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

#[cast_to]
impl LeverDriver for Mouse {
    fn lever(&self) -> i16 {
        self.lever
    }
}

impl Driver for Mouse {}
