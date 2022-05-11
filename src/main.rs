#[macro_use] 
extern crate ioctl_sys;

use std::mem;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;
use std::path::Path;

#[repr(C)]
struct FioExchange<T> {
    error: i32,
    length: usize,
    p_data: *const T,
}

impl<T> FioExchange<T> {
    pub fn new(ptr: *const T) -> Self {
        FioExchange { error: 0, length: mem::size_of::<T>(), p_data: ptr }
    }

    pub fn is_error(&self) -> bool {
        self.error != 0
    }

    pub fn error_code(&self) -> i32 {
        self.error
    }
}

fn ioctl_onu_read<const MAGIC: u32, const TYPE: u32, T: Default>(fd: RawFd) -> Result<T, i32> {
    let data: T = Default::default();
    let fifo = FioExchange::new(&data);

    unsafe {
        let res = libc::ioctl(fd, ior!(MAGIC, TYPE, mem::size_of::<T>()), &fifo);

        if res != 0 {
            return Err(res);
        }
    }

    if fifo.is_error() { 
        return Err(fifo.error_code()) 
    } 

    Ok(data)
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum PloamState {
    PowerUp = 0,
    Initial = 1,
    Standby = 2,
    SerialNumber = 3,
    Ranging = 4,
    Operation = 5,
    POPUP = 6,
    EmergencyStop = 7
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PloamStateData {
    current_state: PloamState,
    previous_state: PloamState,
    elapsed_msec: u32
}

impl ::std::default::Default for PloamStateData {
    fn default() -> Self {
        PloamStateData { current_state: PloamState::PowerUp, previous_state: PloamState::PowerUp, elapsed_msec: 0 }
    }
}

const PLOAM_MAGIC: u32 = 6;
const PLOAM_GET_STATE: u32 = 4;

fn main() {
    let path = Path::new("/dev/onu0");
    let file  = File::open(&path).unwrap();

    let result = ioctl_onu_read::<PLOAM_MAGIC, PLOAM_GET_STATE, PloamStateData>(file.as_raw_fd()).unwrap();

    println!("{:?}", result);
}
