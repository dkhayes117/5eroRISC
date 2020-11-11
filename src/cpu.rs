//use riscv::register::{mepc,mcause,mtval};
use riscv_rt::TrapFrame;
use hifive1::sprintln;
pub const STACK_SIZE: usize = 512;

/// For creating a user mode stack frame
#[repr(C, align(16))]
pub struct StackFrame {
    pub stack: [u8; STACK_SIZE],
}
/// Dumps the registers of a given trap frame. This is NOT the current CPU registers!
pub fn dump_registers(trap_frame: &TrapFrame) {
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

/// Function to cause a stack overflow to test PMP protection
pub fn stack_overflow(){
    unsafe{
        asm!(
            "addi sp, sp, 16",
        )
    }
}

pub fn benchmark(){
    // Function to eliminate integers that are not prime
    fn sieve(primes: &mut [u16], factor: u16) {
        for i in 0..primes.len() {
            let value = primes[i];
            if value != 0 && value != factor {
                if value % factor == 0 {
                    primes[i] = 0;
                }
            }
        }
    }

    // create array for prime sieve
    let mut primes: [u16;500] = [0;500];
    for i in 2..=primes.len() - 1 {
        primes[i] = i as u16;
    }

    for i in 0..primes.len() {
        let factor = primes[i];
        if factor != 0 {
            sieve(&mut primes, factor);
        }
    }
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
