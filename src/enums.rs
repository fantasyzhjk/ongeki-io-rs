

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum OpBtn {
    Test = 0x01,
    Service = 0x02,
    Coin = 0x04,
}


#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum GameBtn {
    Btn1 = 0x01,
    Btn2 = 0x02,
    Btn3 = 0x04,
    Side = 0x08,
    Menu = 0x10,
}

#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum HResult {
    Ok = 0x00,
    Bad = 0x01,
}