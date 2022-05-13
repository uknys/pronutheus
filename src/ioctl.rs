use std::mem;
use std::marker::PhantomData;
use std::os::unix::prelude::RawFd;
use std::os::raw::{c_int, c_ulong};

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

struct IoCtlCommand<const DIRECTION: char, const MAGIC: u32, const TYPE: u32, T> {
    _t: PhantomData<T>,
}

impl<const DIRECTION: char, const MAGIC: u32, const TYPE: u32, T>
    IoCtlCommand<DIRECTION, MAGIC, TYPE, T>
{
    const VALUE: u32 = (TYPE << 0x0)
        | (MAGIC << 0x8)
        | ((::std::mem::size_of::<T>() as u32) << 0x10)
        | (match DIRECTION {
            'N' => 0x0,
            'R' => 0x2,
            'W' => 0x4,
            'X' => 0x6,
            _ => panic!("The Direction can't be anything other than 'R', 'W', 'X' or 'N'."),
        } << 0x1D);
}

#[repr(C)]
struct FioExchange<T> {
    error: i32,
    length: usize,
    p_data: *const T,
}

impl<T> FioExchange<T> {
    pub fn new(ptr: *const T) -> Self {
        FioExchange {
            error: 0,
            length: mem::size_of::<T>(),
            p_data: ptr,
        }
    }

    pub fn is_error(&self) -> bool {
        self.error != 0
    }

    pub fn error_code(&self) -> i32 {
        self.error
    }
}

pub fn request<const DIRECTION: char, const MAGIC: u32, const TYPE: u32, T: Default>(
    fd: RawFd,
) -> Result<T, i32> {
    let data: T = Default::default();
    let fifo = FioExchange::new(&data);

    unsafe {
        let res = ioctl(fd, IoCtlCommand::<DIRECTION, MAGIC, TYPE, T>::VALUE, &fifo);

        if res != 0 {
            return Err(res);
        }
    }

    if fifo.is_error() {
        return Err(fifo.error_code());
    }

    Ok(data)
}