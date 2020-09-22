#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
use core::mem;
//use riscv::register::{};

// This creates a 16 byte aligned memory space for user mode operation
#[repr(align(16))]
struct Align16{ stack: [u8;768] }

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    //Create user stack and determine stack pointer
    let user_stack = Align16{ stack: [0;768] };
    let raw_ptr: *const Align16 = &user_stack;
    let stack_ptr: *const Align16 = unsafe { raw_ptr.offset(1) };

    sprintln!("user_stack alignment: {}", mem::align_of_val(&user_stack));
    sprintln!("user_stack size:      {}", mem::size_of_val(&user_stack));
    sprintln!("memory array pointer: {:p}", &user_stack);
    sprintln!("calc stack pointer:   {:p}", &stack_ptr);
    sprintln!("verify sp stack local:{}", stack_ptr as usize - raw_ptr as usize);

    loop{};
}