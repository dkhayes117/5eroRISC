//! Trap handling

use hifive1::{sprintln};
use riscv::register::{mtval,mcause,mepc, pmpcfg0, pmpcfg1};
use mcause::Trap;
use mcause::Exception::*;
use riscv_rt::TrapFrame;
use crate::pmp::{print_pmp_addresses, print_register_by_byte};
use crate::cpu::dump_registers;

#[export_name = "ExceptionHandler"]
pub fn trap_handler( trap_frame:&TrapFrame ) -> ! {
    //sprintln!("Trap Entered");
    let epc = mepc::read();
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
            mepc::write(epc + 4);
            unsafe{asm!("mret");}
        }
        // If Instruction Fault occurs, like a PMP violation
        Trap::Exception(InstructionFault) => { sprintln!("Instruction Fault Trap Occurred\n") }

        Trap::Exception(exception) => { sprintln!("{:?} Exception Trap Occurred\n",exception)}

        Trap::Interrupt(interrupt) => { sprintln!("{:?} Exception Trap Occurred\n",interrupt)}

    }
    sprintln!("Offending address: {:0X}\n", mtval::read());
    dump_registers(trap_frame);
    print_register_by_byte(pmpcfg0::read(), 0);
    print_register_by_byte(pmpcfg1::read(), 4);
    print_pmp_addresses();

    loop{};
}

