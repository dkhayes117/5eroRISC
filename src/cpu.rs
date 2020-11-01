//use riscv::register::{mepc,mcause,mtval};
use riscv_rt::TrapFrame;

pub const STACK_SIZE: usize = 512;
/// Dumps the registers of a given trap frame. This is NOT the
/// current CPU registers!
pub fn dump_registers(trap_frame: &TrapFrame) {
    use hifive1::sprintln;

    sprintln!("Exception Trap Frame Dump\n");
    sprintln!("ra: {:0X}", trap_frame.ra);
    sprintln!("t0: {:0X}", trap_frame.t0);
    sprintln!("t1: {:0X}", trap_frame.t1);
    sprintln!("t2: {:0X}", trap_frame.t2);
    sprintln!("t3: {:0X}", trap_frame.t3);
    sprintln!("t4: {:0X}", trap_frame.t4);
    sprintln!("t5: {:0X}", trap_frame.t5);
    sprintln!("t6: {:0X}", trap_frame.t6);
    sprintln!("a0: {:0X}", trap_frame.a0);
    sprintln!("a1: {:0X}", trap_frame.a1);
    sprintln!("a2: {:0X}", trap_frame.a2);
    sprintln!("a3: {:0X}", trap_frame.a3);
    sprintln!("a4: {:0X}", trap_frame.a4);
    sprintln!("a5: {:0X}", trap_frame.a5);
    sprintln!("a6: {:0X}", trap_frame.a6);
    sprintln!("a7: {:0X}", trap_frame.a7);
    sprintln!("\nend trap frame\n");
}

// For creating a user mode stack frame
// #[allow(dead_code)]
#[repr(C, align(16))]
pub struct StackFrame {
    pub stack: [u8; STACK_SIZE],
}
/*
impl StackFrame {
    pub const fn new() -> Self {
        StackFrame {
            stack: [0; STACK_SIZE],
        }
    }
}
*/
//store cpu state
/*
#[derive(Clone, Copy)]
pub struct TrapFrame{   // Offsets in bytes
    pub regs: [usize;32],   // 0 - 124
}
impl TrapFrame {
    pub const fn new() -> Self {
        TrapFrame {
            regs: [0; 32],
        }
    }
}
*/
