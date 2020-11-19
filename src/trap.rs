//! Trap handling

use crate::cpu::dump_registers;
use crate::pmp::{print_pmp_addresses, print_register_by_byte};
use hifive1::sprintln;
use mcause::Exception::*;
use mcause::Trap;
use riscv::register::{mcause, mepc, mtval, pmpcfg0, pmpcfg1};
use riscv_rt::TrapFrame;
//use crate::uart::UART_RX;
//use embedded_hal::serial::Read;

#[export_name = "ExceptionHandler"]
pub fn trap_handler(trap_frame: &TrapFrame) {
    let epc = mepc::read();

    match mcause::read().cause() {
        // If u-mode ecall occurs
        Trap::Exception(UserEnvCall) => {
            //sprintln!("User Syscall");
            match trap_frame.a0 {
                1 => {
                    sprintln!("Call type: Exit\n");
                    loop {}
                }
                2 => {
                    //sprintln!("Call type: ConsoleOut");
                    sprintln!("{}", trap_frame.a1);
                    mepc::write(epc + 4);
                    return;
                }
                3 => {
                    use crate::cpu::benchmark;
                    use riscv::register::cycle;
                    let start = cycle::read();
                    benchmark();
                    let end = cycle::read();
                    sprintln!("PMP Benchmarking: {}", end - start);
                    mepc::write(epc + 4);
                    return;
                }
                4 => {
                    use crate::cpu::context_switch;
                    context_switch();
                    mepc::write(epc + 4);
                    return;
                }
                5 => {
                    mepc::write(epc + 4);
                    return;
                }
                6 => {
                    crate::peripherals::read_temp();
                    mepc::write(epc + 4);
                    return;
                }
                _ => {
                    sprintln!("Unknown System Call Detected: {}\n", trap_frame.a0);
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
