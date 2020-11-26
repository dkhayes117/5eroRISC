#![no_std]
#![no_main]
#![feature(asm)]

extern crate panic_halt;

use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};
use riscv::register::{cycle, instret, mcause, mcounteren, mepc, mstatus, pmpaddr0, pmpcfg0};
use riscv_rt::{entry, TrapFrame};

// Number of times to run sieve
const COUNT: usize = 1000;

// This creates a 16 byte aligned memory space for user mode operation
#[repr(C, align(16))]
#[allow(dead_code)]
struct Align16 {
    stack: [u8; 768],
}

fn benchmark() -> usize {
    // Function to eliminate integers that are not prime
    fn sieve(primes: &mut [u16], factor: u16) {
        for i in 0..primes.len() {
            let value = primes[i];
            if value != 0 && value != factor {
                if value % factor == 0 {
                    primes[i] = 0;
                }
            }
        }
    }

    let mut cycles: [usize; COUNT] = [0; COUNT];
    //let mut instructions: usize = 0;

    for i in 0..cycles.len() {
        let start_cycles = cycle::read();
        //let start_instructions = instret::read();
        // create array for prime sieve
        let mut primes: [u16; 250] = [0; 250];
        for i in 2..=primes.len() - 1 {
            primes[i] = i as u16;
        }

        for i in 0..primes.len() {
            let factor = primes[i];
            if factor != 0 {
                sieve(&mut primes, factor);
            }
        }
        //let end_instructions = instret::read();
        let end_cycles = cycle::read();

        let total_cycles = end_cycles - start_cycles;
        //instructions = end_instructions - start_instructions;
        cycles[i] = total_cycles;
    }
    average(cycles)
    //instructions
}

fn average(array: [usize; COUNT]) -> usize {
    array.iter().sum::<usize>() / array.len()
}

// This function handles machine traps due to interrupts or exceptions
#[export_name = "ExceptionHandler"]
fn trap_handler(trap_frame: &TrapFrame) {
    use mcause::Trap;
    match mcause::read().cause() {
        Trap::Exception(exception) => sprintln!("TRAP::Exception Reason::{:?}", exception),
        Trap::Interrupt(interrupt) => sprintln!("{:?} Interrupt Trap Occurred\n", interrupt),
    }
    sprintln!("Avg U-Cycle Count: {}", trap_frame.a0);
    loop {}
}

unsafe fn syscall(_cycles: usize) {
    asm!("ecall");
}

// Function to use as entry for user mode
fn user_mode() {
    let _cache_prime = benchmark();
    let u_bench = benchmark();
    unsafe { syscall(u_bench) };
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    unsafe {
        mcounteren::set_tm();
        mcounteren::set_ir();
    }

    sprintln!("Starting Benchmark Test, Iterations: {}", COUNT);
    let _cache_prime = benchmark();
    let m_bench = benchmark();
    sprintln!("Avg M-Cycle Count: {}", m_bench);

    //Create user stack and determine stack pointer and trap handler
    let user_stack = Align16 { stack: [0; 768] };
    let raw_ptr: *const Align16 = &user_stack;
    let stack_ptr: *const Align16 = unsafe { raw_ptr.offset(1) }; //Top of stack
    let user_entry = user_mode as *const (); //Function address

    //Setup PMP
    let permissions: usize = 0xF; //TOR alignment with RWX permissions
    pmpaddr0::write(0x20001000); // All flash memory available
    pmpcfg0::write(permissions);

    //Setup registers for user mode entry
    mepc::write(user_entry as usize); // Entry point for user mode

    unsafe {
        mcounteren::set_cy();
        mstatus::set_mpp(mstatus::MPP::User);
        mstatus::clear_mie();
        mstatus::set_mpie();

        asm!("mv ra, zero",
        "mv sp, {0}",
        "mret",
        in(reg) stack_ptr);
    };

    loop {}
}
