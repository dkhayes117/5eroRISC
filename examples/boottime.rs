#![no_std]
#![no_main]

/*
* Set pin 9 to high so boot up time can be documented on reset
* with a logic analyzer
*/

extern crate panic_halt;

use hifive1::hal::delay::Sleep;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::pin;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // GPIO PIN1 -> DIG9 physical on board (both hifive1 and hifive1-revB)
    let mut pin9 = pin!(pins, dig9).into_output();

    pin9.toggle().unwrap();

    loop {}
}
