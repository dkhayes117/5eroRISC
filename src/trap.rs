//! Trap handling

use crate::cpu::dump_registers;
use crate::pmp::{print_pmp_addresses, print_register_by_byte};
//use crate::uart::UART_RX;
use mcause::Exception::*;
use mcause::Trap;
use riscv::register::{mcause, mepc, mtval, pmpcfg0, pmpcfg1};
use riscv_rt::TrapFrame;
use hifive1::sprintln;
//use embedded_hal::serial::Read;

#[export_name = "ExceptionHandler"]
pub fn trap_handler(trap_frame: &TrapFrame) {
    let epc = mepc::read();
    match mcause::read().cause() {
        // If u-mode ecall occurs
        Trap::Exception(UserEnvCall) => {
            //sprintln!("User Syscall");
            match trap_frame.a0 {
                0 => {
                    sprintln!("Call type: Exit\n");
                    loop {};
                }
                1 => {
                    //sprintln!("Call type: ConsoleOut");
                    sprintln!("{}", trap_frame.a1);
                    mepc::write(epc + 4);
                    return;
                }
                2 => {
                    use crate::cpu::benchmark;
                    benchmark();
                    mepc::write(epc + 4);
                    return;
                }
                _ => {
                    sprintln!("Unknown System Call Detected\n");
                    panic!();
                }
            }
        }

        Trap::Exception(IllegalInstruction) => {
            sprintln!("Illegal Instruction Trap Occurred");
            sprintln!("Illegal Instruction: {:0X}\n", mtval::read());
        }
/*
        Trap::Exception(Breakpoint) => {
            unsafe{
                sprintln!("Press Any Key to Continue");
                while UART_RX.as_mut().map_or(false, |x| x.read().is_err()){
                    loop{}
                }
            }
            mepc::write(epc + 4);
        }
        */

        Trap::Exception(exception) => sprintln!("{:?} Exception Trap Occurred\n", exception),

        Trap::Interrupt(interrupt) => sprintln!("{:?} Interrupt Trap Occurred\n", interrupt),
    }

    dump_registers(trap_frame);
    print_register_by_byte(pmpcfg0::read(), 0);
    print_register_by_byte(pmpcfg1::read(), 4);
    print_pmp_addresses();

    loop {}
}
