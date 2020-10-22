use riscv::register::{mepc,mstatus};
use riscv::register::mtvec;
use hifive1::sprintln;
use crate::cpu::{StackFrame};
use crate::trap::trap_handler;

pub unsafe fn user_app_entry(user_entry:usize){
        //Create user stack and determine stack pointer and trap handler
        //let mut user_stack = Align16{ stack: [0;768] };
        //let raw_ptr: *const Align16 = &user_stack;
        //let stack_ptr: *const Align16 = raw_ptr.offset(1); //Top of stack

        let mut user_stack = StackFrame::new();
        let raw_ptr = &user_stack as *const StackFrame as *const();
        let stack_ptr = raw_ptr.offset(1);

        sprintln!("stack address::{:0X}", stack_ptr as usize);
        //sprintln!("Trap Address::{:0X}",trap_address);
        sprintln!("User Entry::{:0X}",user_entry);

        let trap_address = trap_handler as *const ();

        //Setup registers for user mode entry
        mepc::write(user_entry as usize);            // Entry point for user mode
        mstatus::set_mpp(mstatus::MPP::User);             //previous privilege set to user
        mtvec::write(trap_address as usize,mtvec::TrapMode::Direct);
        mstatus::clear_mie();
        mstatus::set_mpie();

        asm!(
                "mv ra, zero",
                "mv sp, {0}",
                "mret",
                in(reg) &stack_ptr
        );
    }



