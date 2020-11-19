use hifive1::hal::e310x::{I2C0};
use hifive1::hal::prelude::*;
//use hifive1::hal::serial::Rx;
use hifive1::hal::i2c::{Speed, I2c};
use hifive1::hal::DeviceResources;
use hifive1::pin;
use hifive1::sprintln;
//use hifive1::gpio;
use hifive1::hal::gpio::{NoInvert, IOF0};
use hifive1::hal::gpio::gpio0::{Pin12, Pin13};
//use core::mem::replace;
/*
struct Peripherals{
    i2c: Option<I2c<I2C0, (Pin12<IOF0<NoInvert>>, Pin13<IOF0<NoInvert>>)>>,
}
    impl Peripherals{
        fn take_i2c(&mut self) -> I2c<I2C0, (Pin12<IOF0<NoInvert>>, Pin13<IOF0<NoInvert>>)> {
            let p = replace(&mut self.i2c, None);
            p.unwrap();
        }
    }

pub static mut I2C: Peripherals = Peripherals{
    i2c: Some(I2c<I2C0, (Pin12<IOF0<NoInvert>>, Pin13<IOF0<NoInvert>>)>),
};
*/
//pub static mut UART_RX: Option<Rx<UART0>> = None;
pub static mut TEMP_SENSOR: Option<I2c<I2C0, (Pin12<IOF0<NoInvert>>, Pin13<IOF0<NoInvert>>)>> = None;

pub fn c_to_f(c:u8) -> u8 {
    c * 9 / 5 + 32
}

pub fn config_peripherals() {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(
        p.PRCI,
        p.AONCLK,
        320.mhz().into()
    );

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    // Configure I2C
    let sda = pin!(pins, i2c0_sda).into_iof0();
    let scl = pin!(pins, i2c0_scl).into_iof0();
    //let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);
    unsafe {
        TEMP_SENSOR.replace(
            I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks)
        );
    }
}

pub fn read_temp(){
    let send_buffer = [0x0];
    let mut recv_buffer = [0u8; 0x1];
    unsafe {
        match TEMP_SENSOR.as_mut()
            .unwrap()
            .write_read(0x37, &send_buffer, &mut recv_buffer) {
            Ok(_) => sprintln!("Current Temperature = {:?}°C, {:?}°F", recv_buffer[0], c_to_f(recv_buffer[0])),
            Err(e) => sprintln!("Error: {:?}", e),
        }
    }
}
