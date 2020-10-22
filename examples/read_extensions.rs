#![no_std]
#![no_main]

extern crate panic_halt;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    //Read the misa CSR to see all implemented extensions and base ISA width
    let ext = riscv::register::misa::read();
    sprintln!("Preparing to read MISA register");
    sprintln!("MXL   ZYXWVUTSRQPONMLKJIHGFEDCBA");
    sprintln!("{:032b}", ext.map_or(0, |v| v.bits()));

    loop {}
}
