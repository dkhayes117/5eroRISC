#![no_std]
#![no_main]


extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
use riscv::register::{mepc,mcause,mtval};
//use zerorisc::cpu::*;
use zerorisc::privilege::{drop_privilege_level,ecall};
use zerorisc::pmp::pmp_config;

// This function handles machine traps due to interrupts or exceptions
fn trap_handler(){
    use mcause::Trap;
    let mt_val = mtval::read();
    let exception_pc = mepc::read();
    match mcause::read().cause(){
        Trap::Exception(exception) => {sprintln!("MTRAP::Exception Reason::{:?}",exception)}
        Trap::Interrupt(interrupt) => {sprintln!("MTRAP::Interrupt Reason::{:?}",interrupt)}
    }
    sprintln!("MTVAL Contents::{:0X}",mt_val);
    sprintln!("MEPC Contents::{:0X}",exception_pc);

    loop{};
}

// Function to use as entry for user mode
fn user_mode(){
    //sprintln!("User Mode Entered!");  // Verify that user mode has been entered
    unsafe{ecall()};

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

    let trap_address= trap_handler as *const();
    let user_entry= user_mode as *const();  //Function address

    sprintln!("Trap Address::{:0X}",trap_address as usize);
    sprintln!("User Entry::{:0X}",user_entry as usize);

    pmp_config();

    unsafe{drop_privilege_level( user_entry as usize,trap_address as usize)};

    loop{};
}