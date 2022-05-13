use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::fs::File;

pub mod ploam;
pub mod ioctl;

fn main() {
    let path = Path::new("/dev/onu0");
    let file  = File::open(&path).unwrap();

    let data = crate::ioctl::request::<'R', { crate::ploam::MAGIC }, { crate::ploam::PLOAM_GET_STATE }, crate::ploam::PloamStateData>(file.as_raw_fd()).unwrap();

    println!("{:?}", data);
}
