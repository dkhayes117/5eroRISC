// User Application
use riscv::register::{cycle,instret};
use riscv::asm::ebreak;
use crate::syscall::{syscall, SyscallType};
//use riscv::register::minstret;

pub fn user_app() {
    // Function to eliminate integers that are not prime
    /*
        // Force Stack Overflow
        unsafe{asm!(
        "addi sp, sp, 16",
        )}

        fn overflow() -> usize{
            let x = 1;
            x + 20
        }
        let _z = overflow();
     unsafe {
        syscall(SyscallType::ConsoleOut, total_cycles, total_instructions);
    }
    */

    unsafe{
        ebreak();
        syscall(SyscallType::Exit,0,0);
    }




    //minstret::read();   //Attempt to access so we know we returned to user mode

    loop {}
}
