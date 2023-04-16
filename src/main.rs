#![feature(daisogen_api)]
#![feature(once_cell)]

mod consts;
mod init;
mod ops;

use std::sync::OnceLock;

static IOBASE: OnceLock<u16> = OnceLock::new();

fn main() {
    init::init();

    std::daisogen::yld();
}
