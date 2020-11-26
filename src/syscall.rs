//use core::fmt::Display;

//use riscv_rt::TrapFrame;
pub enum SyscallType {
    Unknown = 0,
    Exit = 1,
    ConsoleOut = 2,
    Benchmark = 3,
    ContextSwitch = 4,
    Syscall = 5,
    GetTemp = 6,
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
