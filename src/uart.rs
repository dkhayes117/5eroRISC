use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin};

pub fn config_uart() {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

// Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

// Configure UART for stdoutcar
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);
}