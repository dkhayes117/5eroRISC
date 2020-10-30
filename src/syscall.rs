//use riscv_rt::TrapFrame;
pub enum SyscallType {
    Exit = 0,
    ConsoleOut = 1,
    Benchmark = 2,
}

pub unsafe fn syscall(_call_type: SyscallType, _param1: usize, _param2: usize) {
    asm!("ecall");
    //let mut trap_frame = &TrapFrame{};
}

pub unsafe fn syscall_asm() {
    asm!("ecall")
}
