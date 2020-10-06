#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
//use core::mem;
use riscv::register::{mcause,mtvec};

// This function handles machine traps due to interrupts or exceptions
fn trap_handler(){
    use mcause::Trap;

    sprintln!("Machine Trap Occurred!");
    match mcause::read().cause(){
        Trap::Exception(exception) => {sprintln!("Exception Reason: {:?}",exception)}
        Trap::Interrupt(interrupt) => {sprintln!("Interrupt Reason: {:?}",interrupt)}
    }
    loop{};
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdoutcar
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);



    // Setup machine trap vector
    let trap_address = trap_handler as *const();
    unsafe{mtvec::write(trap_address as usize,mtvec::TrapMode::Direct)};

    sprintln!("Preparing to Test Trap Handler");


    unsafe{
        asm!("mret");
    }

    loop{};
}