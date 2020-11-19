#![no_std]
#![no_main]

extern crate panic_halt;

//use hifive1::sprintln;
//use riscv::register::{cycle};
use riscv_rt::entry;
use zerorisc::pmp::{Lock, Permission, Pmpconfig, RangeType, pmp_reset};
use zerorisc::privilege::user_app_entry;
use zerorisc::peripherals::{config_peripherals};
use zerorisc::user::user_app;
//use zerorisc::cpu::benchmark;

#[entry]
fn main() -> ! {
    //Setup UART for printing to the console
    config_peripherals();


    // Intialize PMP configurations to 0
    pmp_reset();
/*
       // Benchmarking
    benchmark();
    benchmark();
    let start = cycle::read();
    benchmark();
    let end = cycle::read();
    sprintln!("M-mode cycles {}", end - start);

    read_temp();
    read_temp();
    let start = cycle::read();
    for _i in 0..30 {
        read_temp();
    }
    let end = cycle::read();
    sprintln!("M-mode cycles {}", (end - start)/30);
*/
    // Get addresses for the u-mode entry and the trap vector
    //let trap_address = trap_vector as *const();
    let user_entry = user_app as *const (); //Function address

    //sprintln!("Trap Address::{:0X}",trap_address as usize);
    //sprintln!("User Entry::{:0X}",user_entry as usize);

    // Configure PMP for u-mode permissions

    let pmp0 = Pmpconfig {
        base: 0x2040_0000,
        size: 0x2040_0000,
        range_type: RangeType::TOR,
        pmp_index: 0 as usize,
        permission: Permission::RX,
        locked: Lock::UNLOCKED,
    };

    pmp0.set();

    // Start user app
    unsafe { user_app_entry(user_entry as usize) };

    // Required for no return main()
    loop {}
}
