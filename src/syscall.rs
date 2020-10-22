
pub enum SyscallType {
    Exit = 0,
    PrintToConsole = 1,
    Benchmark = 2,
}

pub unsafe fn syscall(_call_type: SyscallType, _param1: usize, _param2: usize) {
    asm!("ecall");
}

