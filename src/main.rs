#[macro_use]
extern crate arrayref;

mod cartridge;
mod memory;

use cartridge::*;
use memory::*;

fn main() {
    let cart_file = ::std::fs::read("nestest.nes").expect("Unable to read cartridge!");
    let cart = Cart::new(cart_file);
    let _mem = Memory::new(cart.header.mapper());
}
