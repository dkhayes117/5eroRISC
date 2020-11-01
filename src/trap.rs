//! Trap handling

use crate::cpu::dump_registers;
use crate::pmp::{print_pmp_addresses, print_register_by_byte};
use hifive1::sprintln;
use mcause::Exception::*;
use mcause::Trap;
use riscv::register::{mcause, mtval, pmpcfg0, pmpcfg1};
use riscv_rt::TrapFrame;

#[export_name = "ExceptionHandler"]
pub fn trap_handler(trap_frame: &TrapFrame){
        //let epc = mepc::read();
        match mcause::read().cause() {
            // If u-mode ecall occurs
            Trap::Exception(UserEnvCall) => {
                sprintln!("User Syscall");
                match trap_frame.a0 {
                    0 => {
                        sprintln!("Call type: Exit\n");
                    }
                    1 => {
                        sprintln!("Call type: ConsoleOut");
                        sprintln!("Param 1: {}", trap_frame.a1);
                        sprintln!("Param 2:{}\n", trap_frame.a2);
                    }
                    2 => sprintln!("Call type: Benchmark\n"),
                    _ => sprintln!("Unknown System Call Detected\n"),
                }
                //mepc::write(epc + 4);
                //unsafe { asm!("mret"); }
            }

            Trap::Exception(IllegalInstruction) => {
                sprintln!("Illegal Instruction Trap Occurred");
                sprintln!("Illegal Instruction: {:0X}\n", mtval::read());
            }

            Trap::Exception(exception) => sprintln!("{:?} Exception Trap Occurred\n", exception),

            Trap::Interrupt(interrupt) => sprintln!("{:?} Exception Trap Occurred\n", interrupt),
        }


    dump_registers(trap_frame);
    print_register_by_byte(pmpcfg0::read(), 0);
    print_register_by_byte(pmpcfg1::read(), 4);
    print_pmp_addresses();

    loop{};
}
