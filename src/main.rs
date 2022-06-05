use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;

pub mod gtc;
pub mod ioctl;
pub mod lan;
pub mod ploam;

fn main() {
    let path = Path::new("/dev/onu0");
    let file = File::open(&path).unwrap();

    let data = crate::ioctl::request::<
        'X',
        { crate::lan::MAGIC },
        { crate::lan::LAN_GET_PORT },
        crate::lan::LanPortStatus,
    >(file.as_raw_fd())
    .unwrap();

    println!("{:?}", data);

    let data2 = crate::ioctl::request::<
        'R',
        { crate::ploam::MAGIC },
        { crate::ploam::PLOAM_GET_STATE },
        crate::ploam::PloamStateData,
    >(file.as_raw_fd())
    .unwrap();

    println!("{:?}", data2);

    let data3 = crate::ioctl::request::<
        'R',
        { crate::gtc::MAGIC },
        { crate::gtc::GTC_GET_STATUS },
        crate::gtc::GTCStatus,
    >(file.as_raw_fd())
    .unwrap();

    println!("{:?}", data3);
}
