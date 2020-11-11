// User Application

use riscv::register::{instret};
use crate::syscall::{syscall, SyscallType};
//use crate::cpu::stack_overflow;
//use riscv::register::minstret;

pub fn user_app() {
    //stack_overflow();

    // Benchmarking
    let start = instret::read();

    unsafe {
        syscall(SyscallType::Benchmark, 0);
    }

    let end = instret::read();

    unsafe {
        syscall(SyscallType::ConsoleOut, end - start);
    }

    //minstret::read();   //Attempt to access so we know we returned to user mode

    loop {}
}
