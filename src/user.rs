// User Application
//use riscv::register::{instret,cycle};
use crate::syscall::{syscall, SyscallType};
//use crate::cpu::stack_overflow;
//use riscv::register::minstret;

pub fn user_app() {
    //stack_overflow();

    unsafe {
        syscall(SyscallType::ConsoleOut, 1234);
    }

    //minstret::read();   //Attempt to access so we know we returned to user mode

    loop {}
}
