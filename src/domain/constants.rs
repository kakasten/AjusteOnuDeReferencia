//Varieties constants
use std::time::Duration;

pub const PORT:u16 = 22;
pub const IP: &str = "192.168.1.1";
pub const USER_NAME: &str = "admin";
pub const PASSWORD: &str = "intelbras";
pub const MULTIPLE_REGISTER: &[i32] = &[0x90, 0xd0, 0x70];
pub const TIME_OUT: std::time::Duration  = Duration::from_millis(650);