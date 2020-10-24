#![no_std]
#![no_main]

extern crate panic_halt;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};
use riscv::register::{
    pmpaddr0, pmpaddr1, pmpaddr2, pmpaddr3, pmpaddr4, pmpaddr5, pmpaddr6, pmpaddr7, pmpcfg0,
    pmpcfg1,
};
use riscv_rt::entry;

fn print_register_by_byte(x: usize, mut i: i32) {
    sprintln!("         L  A-XWR");
    for byte in x.to_be_bytes().iter().rev() {
        sprintln!("pmp{}cfg: {:08b}", i, byte);
        i += 1;
    }
}

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

    pmpaddr0::write(0x2040_0000); // All memory can be accessed
    pmpaddr1::write(0x2000_03FF); // All of RAM is accessable
    pmpcfg0::write(0xF0F);

    sprintln!("Preparing to read PMP registers");

    // Read the pmp registers
    let pmp0 = pmpaddr0::read();
    let pmp1 = pmpaddr1::read();
    let pmp2 = pmpaddr2::read();
    let pmp3 = pmpaddr3::read();
    let pmp4 = pmpaddr4::read();
    let pmp5 = pmpaddr5::read();
    let pmp6 = pmpaddr6::read();
    let pmp7 = pmpaddr7::read();

    let pmp_cfg0 = pmpcfg0::read();
    let pmp_cfg1 = pmpcfg1::read();

    // Print the PMP address registers in hex
    sprintln!("pmpaddr0: {:#X}", pmp0);
    sprintln!("pmpaddr1: {:#X}", pmp1);
    sprintln!("pmpaddr2: {:#X}", pmp2);
    sprintln!("pmpaddr3: {:#X}", pmp3);
    sprintln!("pmpaddr4: {:#X}", pmp4);
    sprintln!("pmpaddr5: {:#X}", pmp5);
    sprintln!("pmpaddr6: {:#X}", pmp6);
    sprintln!("pmpaddr7: {:#X}", pmp7);

    //sprintln!("pmpcfg0: {:#X}", _pmp_cfg0);
    // Print the PMP configuration registers 8 bits, or byte, at a time
    print_register_by_byte(pmp_cfg0, 0);
    print_register_by_byte(pmp_cfg1, 4);

    loop {}
}
