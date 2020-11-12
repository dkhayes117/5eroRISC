// User Application

use riscv::register::{cycle};
use crate::syscall::{syscall, SyscallType};
//use crate::cpu::stack_overflow;
//use riscv::register::minstret;

pub fn user_app() {
    //stack_overflow();

    // Benchmarking
    let start = cycle::read();
    unsafe { syscall(SyscallType::Syscall, 0) }
    //unsafe { syscall(SyscallType::ContextSwitch, 0) }
    let end = cycle::read();

    unsafe {
        syscall(SyscallType::ConsoleOut, end - start);
        //syscall(SyscallType::Unknown, 0);
    }

    //minstret::read();   //Attempt to access so we know we returned to user mode

    loop {}
}
