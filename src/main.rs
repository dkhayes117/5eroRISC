#![no_std]
#![no_main]

extern crate panic_halt;

//use hifive1::sprintln;
//use riscv::register::{mcause, mepc, mtval};
use riscv_rt::entry;
use zerorisc::pmp::{Pmpconfig,RangeType, Permission, Lock};
use zerorisc::privilege::user_app_entry;
use zerorisc::uart::config_uart;
use zerorisc::user::user_app;

#[entry]
fn main() -> ! {
    //Setup UART for printing to the console
    config_uart();

    // Get addresses for the u-mode entry and the trap vector
    //let trap_address = trap_vector as *const();
    let user_entry = user_app as *const(); //Function address

    //sprintln!("Trap Address::{:0X}",trap_address as usize);
    //sprintln!("User Entry::{:0X}",user_entry as usize);

    // Configure PMP for u-mode permissions

    let pmp0 = Pmpconfig{
        base: 0x2040_0000,
        size: 0x2040_0000,
        range_type: RangeType::TOR,
        pmp_index: 0 as usize,
        permission: Permission::RWX,
        locked: Lock::UNLOCKED
    };

    pmp0.set();



    // Start user app
    unsafe { user_app_entry(user_entry as usize) };

    // Required for no return main()
    loop {}
}
