#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{sprintln, pin};
use riscv::register::{mcycle, minstret};

// Function to eliminate integers that are not prime
fn sieve(primes:&mut [u16], factor:u16){
    for i in 0..primes.len(){
        let value = primes[i];
        if value != 0 && value != factor{
            if value % factor == 0{
                primes[i] = 0;
            }
        }
    }
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(p.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), clocks);

    //Enable and initialize performance counter
    let start_cycles = mcycle::read();
    let start_instructions = minstret::read();

    // create array for prime sieve

    let mut primes: [u16;1000] = [0;1000];
    for i in 2..=primes.len()-1{
            primes[i] = i as u16;
    }

    for i in 0..primes.len(){
        let factor = primes[i];
        if factor != 0 {
                sieve(&mut primes, factor);
        }
    }

    let end_cycles = mcycle::read();
    let end_instructions = minstret::read();

    let total_cycles = end_cycles - start_cycles;
    let total_instructions = end_instructions - start_instructions;

    sprintln!("Cycle Count: {}, Instruction Count: {}",total_cycles,total_instructions);

    loop {}
}