#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::i2c::{Speed, I2c};
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};


#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 100.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    // Configure I2C
    let sda = pin!(pins, i2c0_sda).into_iof0();
    let scl = pin!(pins, i2c0_scl).into_iof0();
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    /* Read calibration data from PCT2075 sensor
    * i2c address default 0x37 (configured by pins A0-A2)
    * Read only temperature register 0x00
    * Configuration Register 0x01
    * Hysterisis register 0x02
    * OS register 0x03
    * Measurement idle time configuration register 0x04
    */

    let send_buffer = [0x0];
    let mut recv_buffer = [0u8; 0x1];

        match i2c.write_read(0x37, &send_buffer, &mut recv_buffer) {
            Ok(_) => sprintln!("Current Temperature = {:?}°C", recv_buffer[0]),
            Err(e) => sprintln!("Error: {:?}", e),
        }
    loop {}
}