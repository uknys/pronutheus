pub const MAGIC: u32 = 3;
pub const GTC_GET_STATUS: u32 = 4;

#[repr(C)]
#[derive(Debug)]
pub enum GTCState {
    Hunt = 0,
    Presync = 1,
    Sync = 3,
}

#[repr(C)]
#[derive(Debug)]
pub struct GTCStatus {
    ds_fec_enable: u32,
    us_fec_enable: u32,
    ds_ploam_waiting: u32,
    ds_ploam_overflow: u32,
    ds_state: GTCState,
    ds_sf_state: GTCState,
    ds_physical_equipment_error: u32,
    onu_id: u32,
    gtc_ds_delay: u32,
}

impl ::std::default::Default for GTCStatus {
    fn default() -> Self {
        GTCStatus {
            ds_fec_enable: 0,
            us_fec_enable: 0,
            ds_ploam_waiting: 0,
            ds_ploam_overflow: 0,
            ds_state: GTCState::Hunt,
            ds_sf_state: GTCState::Hunt,
            ds_physical_equipment_error: 0,
            onu_id: 0,
            gtc_ds_delay: 0,
        }
    }
}
