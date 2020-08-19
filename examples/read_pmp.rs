#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
use riscv::register::{pmpaddr0,pmpaddr1,pmpaddr2,pmpaddr3,pmpaddr4,pmpaddr5,pmpaddr6,pmpaddr7,
                      pmpcfg0,pmpcfg1};

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    sprintln!("Preparing to read PMP registers");

    // Read the pmp registers
    let _pmp0 = pmpaddr0::read();
    let _pmp1 = pmpaddr1::read();
    let _pmp2 = pmpaddr2::read();
    let _pmp3 = pmpaddr3::read();
    let _pmp4 = pmpaddr4::read();
    let _pmp5 = pmpaddr5::read();
    let _pmp6 = pmpaddr6::read();
    let _pmp7 = pmpaddr7::read();

    let _pmp_cfg0 = pmpcfg0::read();
    let _pmp_cfg1 = pmpcfg1::read();

    // Print the PMP address registers read
    sprintln!("pmpaddr0: {:032b}", _pmp0);
    sprintln!("pmpaddr1: {:032b}", _pmp1);
    sprintln!("pmpaddr2: {:032b}", _pmp2);
    sprintln!("pmpaddr3: {:032b}", _pmp3);
    sprintln!("pmpaddr4: {:032b}", _pmp4);
    sprintln!("pmpaddr5: {:032b}", _pmp5);
    sprintln!("pmpaddr6: {:032b}", _pmp6);
    sprintln!("pmpaddr7: {:032b}", _pmp7);
    sprintln!("pmpcfg0:  {:032b}", _pmp_cfg0);
    sprintln!("pmpcfg1:  {:032b}", _pmp_cfg1);

    loop {}
}