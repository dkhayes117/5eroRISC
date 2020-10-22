//! Trap handling

use hifive1::{sprintln};
use riscv::register::{mtval,mcause};
use mcause::Trap;
use mcause::Exception::*;
use riscv_rt::TrapFrame;
use crate::cpu::dump_registers;

#[export_name = "ExceptionHandler"]
pub fn trap_handler( trap_frame:&TrapFrame ) -> ! {
    //sprintln!("Trap Entered");

    match mcause::read().cause() {
        // If u-mode ecall occurs
        Trap::Exception(UserEnvCall) => {
            match trap_frame.a0{
                0 => {sprintln!("Call type: Exit\n")}
                1 => {
                    sprintln!("Call type: PrintToConsole\n");
                    //let sys_param1 = trap_frame.a1;
                    //let sys_param2 = trap_frame.a2;
                }
                2 => {sprintln!("Call type: Benchmark\n")}
                _ => {sprintln!("Unknown System Call Detected\n")}
            }
        }
        // If Instruction Fault occurs, like a PMP violation
        Trap::Exception(InstructionFault) => {
            sprintln!("Instruction Fault Trap Occurred \
                       Offending address: {}\n", mtval::read())
        }

        Trap::Exception(exception) => { sprintln!("{:?} Exception Trap Occurred\n",exception); panic!() }

        Trap::Interrupt(interrupt) => { sprintln!("{:?} Exception Trap Occurred\n",interrupt); panic!() }

    }
    dump_registers(trap_frame);

    loop{};
}

