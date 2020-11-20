// User Application

use crate::syscall::{syscall, SyscallType};
//use crate::cpu::stack_overflow;
//use riscv::register::cycle;
//use crate::pmp::pmp_reset;

pub fn user_app() {
    /******* Security testing code **************************************/
    //stack_overflow();
    //pmp_reset();   //Attempt to turn PMP off
    //unsafe { syscall(0, 0); }     //Unknown syscall
    //minstret::read();
    /*******************************************************************/

    /******* Benchmarking **********************************************/
        // Benchmarking
    /*
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
    unsafe { syscall(SyscallType::GetTemp, 0) }
    unsafe { syscall(SyscallType::GetTemp, 0) }
    /******* User Programming ******************************************/
    let start = cycle::read();

    for _i in 0..30{
        unsafe {
            syscall(SyscallType::GetTemp, 0);
        }
    }

    let end = cycle::read();

    unsafe { syscall(SyscallType::ConsoleOut, (end - start)/30); }
    /*******************************************************************/
*/
    unsafe { syscall(SyscallType::GetTemp, 0) }

    loop {}
}
