// User Application

use crate::syscall::{syscall, SyscallType};
use riscv::register::cycle;
use crate::cpu::stack_overflow;
//use crate::pmp::pmp_reset;

pub fn user_app() {
    /******* Security testing code **************************************/
    //stack_overflow();
    //pmp_reset();   //Attempt to turn PMP off
    //unsafe { syscall(SyscallType::Unknown, 0); }
    //minstret::read();
    /*******************************************************************/

    /******* Benchmarking **********************************************/
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
    /*******************************************************************/

    /******* User Programming ******************************************/

    /*******************************************************************/

    loop {}
}
