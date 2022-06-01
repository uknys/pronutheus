#![allow(non_camel_case_types)]

pub const MAGIC: u32 = 7;
pub const LAN_GET_PORT: u32 = 9;

#[repr(C)]
#[derive(Debug)]
pub enum LanModeInterface {
    OFF = 0,
    GPHY = 1,
    EPHY = 2,
    SGMII_SLOW = 3,
    SGMII_FAST = 4,
    RGMII_MAC  = 5,
    RMII_MAC   = 6,
    RMII_PHY   = 7,
    GMII_MAC   = 8,
    GMII_PHY   = 9,
    MII_MAC    = 10,
    MII_PHY    = 11,
    TMII_MAC   = 12,
    TMII_PHY   = 13,
    TBI_MAC    = 14,
    TBI_PHY    = 15
}

#[repr(C)]
#[derive(Debug)]
pub enum LanPhyStatus {
    Off = 0,
    Down = 1,
    Up10 = 2,
    Up100 = 3,
    Up1000 = 4,
    None = 5,
    Unknown = 255,
}

#[repr(C)]
#[derive(Debug)]
pub enum LanModeSpeed {
    Auto = 0,
    Speed10 = 1,
    Speed100 = 2,
    Speed200 = 3,
    Speed1000 = 4,
    Speed2500 = 5
}

#[repr(C)]
#[derive(Debug)]
pub enum LanModeDuplex {
    Auto = 0,
    Full = 1,
    Half = 2
}

#[repr(C)]
#[derive(Debug)]
pub struct LanPortStatus {
    index: u32,
    mode: LanModeInterface,
    port_enable: u32,
    link_status: LanModeSpeed,
    phy_duplex: LanModeDuplex
}

impl ::std::default::Default for LanPortStatus {
    fn default() -> Self {
        LanPortStatus { index: 0u32, mode: LanModeInterface::OFF, port_enable: 0, link_status: LanModeSpeed::Auto, phy_duplex: LanModeDuplex::Auto }
    }
}    