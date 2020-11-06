#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};
use riscv_rt::entry;
//use core::mem;
use riscv::register::{mcause, mepc, mstatus, mtval, mtvec, pmpaddr0, pmpcfg0};

// This creates a 16 byte aligned memory space for user mode operation
#[repr(C, align(16))]
#[allow(dead_code)]
struct Align16 {
    stack: [u8; 768],
}

// This function handles machine traps due to interrupts or exceptions
fn trap_handler() {
    use mcause::Trap;
    let mt_val = mtval::read();
    let exception_pc = mepc::read();
    match mcause::read().cause() {
        Trap::Exception(exception) => sprintln!("MTRAP::Exception Reason::{:?}", exception),
        Trap::Interrupt(interrupt) => sprintln!("MTRAP::Interrupt Reason::{:?}", interrupt),
    }
    sprintln!("MTVAL Contents::{:0X}", mt_val);
    sprintln!("MEPC Contents::{:0X}", exception_pc);

    loop {}
}

// Function to use as entry for user mode
fn user_mode() {
    //sprintln!("User Mode Entered!");  // Verify that user mode has been entered
    unsafe { asm!("ecall") };

    loop {}
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdoutcar
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    //Create user stack and determine stack pointer and trap handler
    let mut user_stack = Align16 { stack: [0; 768] };
    let raw_ptr: *const Align16 = &user_stack;
    let stack_ptr: *const Align16 = unsafe { raw_ptr.offset(1) }; //Top of stack
    let trap_address = trap_handler as *const ();
    let user_entry = user_mode as *const (); //Function address

    //sprintln!("aligned array address:{:p}", raw_ptr);

    //sprintln!("user_stack alignment: {}", mem::align_of_val(&user_stack));
    //sprintln!("user_stack size:      {}", mem::size_of_val(&user_stack));

    sprintln!("stack address::{:0X}", stack_ptr as usize);
    sprintln!("Trap Address::{:0X}", trap_address as usize);
    sprintln!("User Entry::{:0X}", user_entry as usize);

    //Setup PMP
    let permissions: usize = 0xF; //TOR alignment with RWX permissions
    pmpaddr0::write(0x20400000); // All flash memory available
    pmpcfg0::write(permissions);

    //Setup registers for user mode entry
    mepc::write(user_entry as usize); // Entry point for user mode

    unsafe {
        mstatus::set_mpp(mstatus::MPP::User);
        mtvec::write(trap_address as usize, mtvec::TrapMode::Direct);
        mstatus::clear_mie();
        mstatus::set_mpie();

        asm!("mv ra, zero",
        "mv sp, {0}",
        "mret",
        in(reg) stack_ptr);
    };

    loop {}
}
