#![allow(non_camel_case_types)]

pub const MAGIC: u32 = 6;
pub const PLOAM_GET_STATE: u32 = 4;

#[repr(C)]
#[derive(Debug)]
pub enum PloamState {
    PowerUp = 0,
    Initial = 1,
    Standby = 2,
    SerialNumber = 3,
    Ranging = 4,
    Operation = 5,
    POPUP = 6,
    EmergencyStop = 7,
}

#[repr(C)]
#[derive(Debug)]
pub struct PloamStateData {
    current_state: PloamState,
    previous_state: PloamState,
    elapsed_msec: u32,
}

impl ::std::default::Default for PloamStateData {
    fn default() -> Self {
        PloamStateData {
            current_state: PloamState::Operation,
            previous_state: PloamState::Ranging,
            elapsed_msec: 0,
        }
    }
}
