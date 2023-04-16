use super::consts::*;
use std::daisogen::asm;

pub fn init() {
    // Device available?
    let dev = std::daisogen::pd_call2("pci_query_vendor", PCI_VENDOR_ID, PCI_DEVICE_ID);
    let dev: Vec<u32> = std::daisogen::deserialize(dev as usize);
    if dev.len() == 0 {
        debug!("rtl8139: no devices");
        std::daisogen::yld(); // TODO: Change to exit()
    }
    let dev = dev[0];

    // Get iobase
    let iobase = std::daisogen::pd_call2("pci_get_bar", dev as u64, 0) as u16;
    assert_ne!(iobase, 0);
    super::IOBASE.get_or_init(|| iobase);

    // Enable bus mastering
    std::daisogen::pd_call2("pci_set_bus_master", dev as u64, 1);

    // ---

    // Turn it on (LWAKE + LWPTN)
    asm::out8(iobase + REG_CONFIG_1, 0x0);

    // Software reset
    asm::out8(iobase + REG_CMD, 0x10);
    while (asm::in8(iobase + REG_CMD) & 0x10) != 0 {}

    // Initialize recv buffer, 3 pages (>= 8KB + 16B recommended)
    let recv = std::daisogen::phys_alloc(3).unwrap();
    assert_eq!(recv, recv & ((1 << 32) - 1)); // Oops
    asm::out32(iobase + REG_RBSTART, recv as u32);

    // Set Interrupt Mask Register + Interrupt Service Register
    asm::out16(iobase + REG_IMR, 0x0005); // TOK + ROK

    // Configure receive buffer
    asm::out32(iobase + REG_RCR, 0xF | (1 << 7)); // TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    // Enable receive and transmitter
    asm::out8(iobase + REG_CMD, 0x0C); // RE + TE
}
