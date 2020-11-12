//use core::fmt::Display;

//use riscv_rt::TrapFrame;
pub enum SyscallType {
    Exit = 0,
    ConsoleOut = 1,
    Benchmark = 2,
    ContextSwitch = 3,
    Syscall = 4,
    Unknown = 5
}

pub unsafe fn syscall (_call_type: SyscallType, _x: usize) {
    asm!("ecall");
}

