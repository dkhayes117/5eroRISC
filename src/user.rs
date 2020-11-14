// User Application

use crate::syscall::{syscall, SyscallType};
use riscv::register::cycle;
//use crate::cpu::stack_overflow;
//use riscv::register::minstret;

pub fn user_app() {
    //stack_overflow();

    // Benchmarking
    unsafe { syscall(SyscallType::Syscall, 0) }
    unsafe { syscall(SyscallType::Syscall, 0) }

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
