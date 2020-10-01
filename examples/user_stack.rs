#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
//use core::mem;
use riscv::register::{mepc,mstatus,mcause,mtvec};

// This creates a 16 byte aligned memory space for user mode operation
/*
#[repr(align(16))]
struct Align16{ stack: [u8;768] }
*/
// Function to use as entry for user mode
fn user_mode(){
    sprintln!("User Mode Entered!");  // Verify that user mode has been entered
    let _attempt = mepc::read();
    loop{};
}
// This function handles machine traps from user mode due to interrupts or exceptions
fn trap_handler(){
    sprintln!("Machine Trap Occurred!");
    let cause = mcause::read().code();
    //let ext = misa::read();
    //sprintln!("{:032b}", ext.map_or(0, |v| v.bits()));
    sprintln!("Trap Cause: {}",cause);

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

    //Create user stack and determine stack pointer
    //let mut user_stack = Align16{ stack: [0;768] };
    //let raw_ptr: *const Align16 = &user_stack;
    //let stack_ptr: *const Align16 = raw_ptr.offset(1);

    // Top of stack using all 16k of ram
    let stack_ptr: usize = 0x80004000;

    //sprintln!("user_stack alignment: {}", mem::align_of_val(&user_stack));
    //sprintln!("user_stack size:      {}", mem::size_of_val(&user_stack));
    //sprintln!("memory array pointer: {:p}", raw_ptr);
    //sprintln!("calc stack pointer:   {:p}", stack_ptr);
    //sprintln!("verify sp stack local:{}", stack_ptr as usize - raw_ptr as usize);

    // Setup machine trap vector
    let trap_address = trap_handler as *const();
    unsafe{mtvec::write(trap_address as usize,mtvec::TrapMode::Direct)};

    // prepare to entry user mode
    let user_entry = user_mode as *const();
    mepc::write(user_entry as usize);  // Entry point for user mode
    unsafe{mstatus::set_mpp(mstatus::MPP::User)}; // Set MPP bit to enter user mode (00)

    sprintln!("Preparing to Enter User Mode");
    //Add entry point to MEPC and set MPP in MSTATUS to 00
    //#[cfg(all(riscv, feature = "inline-asm"))]
    unsafe{
        asm!("mv ra, zero",
             "mv sp, {0}",
             "mret",
             in(reg) stack_ptr);
    }


    loop{};
}