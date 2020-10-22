// User Application
use riscv::register::minstret;
use crate::syscall::{syscall,SyscallType};


pub fn user_app(){
    //sprintln!("User Mode Entered!");  // Verify that user mode has been entered
    //let p1: usize = 22;
    //let p2: usize = 44;

    //minstret::read();   //Attempt to access so we know we returned to user mode

    //unsafe{syscall(SyscallType::PrintToConsole,p1, p2);}

    unsafe{asm!("ecall")};
    minstret::read();   //Attempt to access so we know we returned to user mode

    loop{};
}