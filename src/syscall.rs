//use core::fmt::Display;

//use riscv_rt::TrapFrame;
pub enum SyscallType {
    Exit = 0,
    ConsoleOut = 1,
    Benchmark = 2,
    ContextSwitch = 3,
    Syscall = 4,
    Unknown = 5,
}
/// Request Action from M-Mode
#[inline(never)]
pub unsafe fn syscall(call_type: SyscallType, x: usize) {

    asm!(
        "mv a0, {0}",
        "addi a1, {1}, 0",
        "ecall",
         in(reg) call_type as usize,
         in(reg) x,
    );
}
