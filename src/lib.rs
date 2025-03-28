#![allow(clippy::not_unsafe_ptr_arg_deref)]

use drivers::Drivers;
use lazy_static::lazy_static;
use std::sync::RwLock;
use windows::Win32::System::Console;

use enums::HResult;

mod config;
mod drivers;
mod enums;

lazy_static! {
    static ref DRIVERS: RwLock<Drivers> = RwLock::new(Drivers::new());
}

#[no_mangle]
pub extern "C" fn mu3_io_get_api_version() -> u16 {
    0x0100
}

#[no_mangle]
pub extern "C" fn mu3_io_init() -> HResult {
    unsafe { let _ = Console::AttachConsole(Console::ATTACH_PARENT_PROCESS); }
    std::panic::set_hook(Box::new(|panic_info| {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);

        println!("按回车退出");
        std::io::stdin().read_line(&mut String::new()).unwrap();

        std::process::exit(1);
    }));

    println!("Ongeki IO: 启动！");

    let mut drivers = DRIVERS.write().unwrap();
    drivers.init();

    HResult::Ok
}

#[no_mangle]
pub extern "C" fn mu3_io_poll() -> HResult {
    let mut drivers = DRIVERS.write().unwrap();
    drivers.poll();

    HResult::Ok
}

#[no_mangle]
pub extern "C" fn mu3_io_get_opbtns(option_btns: *mut u8) {
    let drivers = DRIVERS.read().unwrap();
    if !option_btns.is_null() {
        unsafe { *option_btns = drivers.op_btns() }
    }
}

#[no_mangle]
pub extern "C" fn mu3_io_get_gamebtns(left: *mut u8, right: *mut u8) {
    let drivers = DRIVERS.read().unwrap();
    if !left.is_null() {
        unsafe { *left = drivers.left_btns() }
    }
    if !right.is_null() {
        unsafe { *right = drivers.right_btns() }
    }
}

#[no_mangle]
pub extern "C" fn mu3_io_get_lever(pos: *mut i16) {
    let drivers = DRIVERS.read().unwrap();
    if !pos.is_null() {
        unsafe { *pos = drivers.lever().unwrap_or(0) }
    }
}

#[no_mangle]
pub extern "C" fn mu3_io_led_init() -> HResult {
    HResult::Ok
}

#[no_mangle]
pub extern "C" fn mu3_io_set_led(data: u32) {
    let mut drivers = DRIVERS.write().unwrap();
    drivers.set_led(data);
}

#[no_mangle]
pub extern "C" fn mu3_io_led_set_colors(board: u8, rgb: *mut u8) {
    let mut drivers = DRIVERS.write().unwrap();
    let data =  unsafe { std::slice::from_raw_parts(rgb, 61) };
    drivers.set_led_new(board, data);
}

#[cfg(test)]
mod tests {

    use super::drivers::hid;

    #[test]
    fn map_test() {
        let temp = hid::map(12, -280, 280, -20000, 20000);
        assert_eq!(temp, 857);
    }

}
