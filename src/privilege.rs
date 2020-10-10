use riscv::register::{mepc,mstatus,mtvec};
use hifive1::{sprintln};
#[allow(dead_code)]
#[repr(align(16))]
struct Align16{ stack: [u8;768] }

pub unsafe fn drop_privilege_level(user_entry:usize,trap_address:usize){
        //Create user stack and determine stack pointer and trap handler
        let mut user_stack = Align16{ stack: [0;768] };
        let raw_ptr: *const Align16 = &user_stack;
        let stack_ptr: *const Align16 = raw_ptr.offset(1); //Top of stack

        sprintln!("stack address::{:0X}", stack_ptr as usize);
        sprintln!("Trap Address::{:0X}",trap_address);
        sprintln!("User Entry::{:0X}",user_entry);

        //Setup registers for user mode entry
        mepc::write(user_entry as usize);            // Entry point for user mode
        mstatus::set_mpp(mstatus::MPP::User);             //previous privilege set to user
        mtvec::write(trap_address as usize,mtvec::TrapMode::Direct);
        mstatus::clear_mie();
        mstatus::set_mpie();

        asm!("mv ra, zero",
        "mv sp, {0}",
        "mret",
        in(reg) &stack_ptr);
    }

pub unsafe fn ecall(){
        asm!("ecall");
    }

