pub const PCI_VENDOR_ID: u64 = 0x10EC;
pub const PCI_DEVICE_ID: u64 = 0x8139;

pub const REG_RBSTART: u16 = 0x30;
pub const REG_CMD: u16 = 0x37;
pub const REG_IMR: u16 = 0x3C;
pub const REG_RCR: u16 = 0x44;
pub const REG_CONFIG_1: u16 = 0x52;

pub const TX_CMD: [u16; 4] = [0x10, 0x14, 0x18, 0x1C];
pub const TX_START: [u16; 4] = [0x20, 0x24, 0x28, 0x2C];
pub const MAX_DATA_SIZE: u16 = 1792;

bitfield::bitfield! {
    pub struct TX(u32);
    u32;

    pub get_size, set_size: 12, 0;
    pub get_own, set_own: 13;
    pub get_tok, set_tok: 15;
}

impl TX {
    pub fn raw(&self) -> u32 {
        self.0
    }
}
