use super::consts::*;
use std::daisogen::asm;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// This currently just handles one packet at a time
// This is sync at the time!
static GUARD: Mutex<()> = Mutex::new(());
static CTR: AtomicUsize = AtomicUsize::new(0);
pub extern "C" fn transmit(ptr: usize, sz: usize) {
    assert_eq!(ptr, ptr & ((1 << 32) - 1));
    let guard = GUARD.lock().unwrap();
    let ctr = CTR.load(Ordering::SeqCst);
    let (cmd, start) = (TX_CMD[ctr], TX_START[ctr]);
    let iobase = super::IOBASE.get().unwrap();

    asm::out32(iobase + start, ptr as u32);

    let mut tx: TX = TX(0);
    tx.set_size(sz as u32);
    tx.set_own(false); // Go!
    tx.set_tok(false);
    asm::out32(iobase + cmd, tx.raw());

    // Sync wait (terrible, but temporal)
    loop {
        let tx: TX = TX(asm::in32(iobase + cmd));
        if tx.get_tok() {
            break;
        }
    }
}
