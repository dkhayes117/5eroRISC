/*
pub struct StoredState{
    regs: [usize;31],   //store cpu registers
    pc: usize,          //pc value of u-mode at exception/interrupt
    mcause: usize,      //trap cause
    mtval: usize,       //In case of fault
}
*/
pub unsafe fn save_state() {
    //x0:zero, x1:ra, x2:sp
    asm!("addi sp, sp, -34*4",   //Make room on stack
    "sw   x3,  4*4(sp)",    // store CPU registers
    "sw   x4,  5*4(sp)",
    "sw   x5,  6*4(sp)",
    "sw   x6,  7*4(sp)",
    "sw   x7,  8*4(sp)",
    "sw   x8,  9*4(sp)",
    "sw   x9,  10*4(sp)",
    "sw   x10, 11*4(sp)",
    "sw   x11, 12*4(sp)",
    "sw   x12, 13*4(sp)",
    "sw   x13, 14*4(sp)",
    "sw   x14, 15*4(sp)",
    "sw   x15, 16*4(sp)",
    "sw   x16, 17*4(sp)",
    "sw   x17, 18*4(sp)",
    "sw   x18, 19*4(sp)",
    "sw   x19, 20*4(sp)",
    "sw   x20, 21*4(sp)",
    "sw   x21, 22*4(sp)",
    "sw   x22, 23*4(sp)",
    "sw   x23, 24*4(sp)",
    "sw   x24, 25*4(sp)",
    "sw   x25, 26*4(sp)",
    "sw   x26, 27*4(sp)",
    "sw   x27, 28*4(sp)",
    "sw   x28, 29*4(sp)",
    "sw   x29, 30*4(sp)",
    "sw   x30, 31*4(sp)",
    "sw   x31, 32*4(sp)")
}

pub unsafe fn restore_state(return_ptr:usize){
        asm!("mv   t0,  {0}",       // Save the state pointer to a specific register.
        "lw   x1,  0*4(t0)", // ra
        "lw   x2,  1*4(t0)",  // sp
        "lw   x3,  2*4(t0)",  // gp
        "lw   x4,  3*4(t0)",  // tp
        "lw   x6,  5*4(t0)",  // t1
        "lw   x7,  6*4(t0)",  // t2
        "lw   x8,  7*4(t0)",  // s0,fp
        "lw   x9,  8*4(t0)",  // s1
        "lw   x10, 9*4(t0)",  // a0
        "lw   x11, 10*4(t0)", // a1
        "lw   x12, 11*4(t0)", // a2
        "lw   x13, 12*4(t0)", // a3
        "lw   x14, 13*4(t0)", // a4
        "lw   x15, 14*4(t0)", // a5
        "lw   x16, 15*4(t0)", // a6
        "lw   x17, 16*4(t0)", // a7
        "lw   x18, 17*4(t0)", // s2
        "lw   x19, 18*4(t0)", // s3
        "lw   x20, 19*4(t0)", // s4
        "lw   x21, 20*4(t0)", // s5
        "lw   x22, 21*4(t0)", // s6
        "lw   x23, 22*4(t0)", // s7
        "lw   x24, 23*4(t0)", // s8
        "lw   x25, 24*4(t0)", // s9
        "lw   x26, 25*4(t0)", // s10
        "lw   x27, 26*4(t0)", // s11
        "lw   x28, 27*4(t0)", // t3
        "lw   x29, 28*4(t0)", // t4
        "lw   x30, 29*4(t0)", // t5
        "lw   x31, 30*4(t0)", // t6
        "lw   x5,  4*4(t0)",
        in(reg) return_ptr)
}

