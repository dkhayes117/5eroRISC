use core::mem::size_of_val;
use crate::cpu::{StackFrame};
use crate::pmp::{Lock, Permission, Pmpconfig, RangeType};
//use hifive1::sprintln;
use riscv::register::{mcounteren, mepc, mstatus};
//use crate::pmp::napot_range;

pub unsafe fn user_app_entry(user_entry: usize) {
    //Create user stack and determine stack pointer and trap handler
    let user_stack = StackFrame::new();
    let raw_ptr: *const StackFrame = &user_stack;
    let stack_ptr: *const StackFrame = raw_ptr.offset(1); //Top of stack

    //sprintln!("bottom of stack frame::{:0X}", raw_ptr as usize);
    //sprintln!("top of stack frame::{:0X}", stack_ptr as usize);

    //sprintln!("Trap Address::{:0X}",trap_address);
    //sprintln!("User Entry::{:0X}", user_entry);

    //let trap_address = trap_handler as *const ();

    let pmp1 = Pmpconfig {
        base: raw_ptr as usize, //raw_ptr as usize
        size: size_of_val(&user_stack),
        range_type: RangeType::TOR,
        pmp_index: 1 as usize,
        permission: Permission::RW,
        locked: Lock::UNLOCKED,
    };

    let pmp2 = Pmpconfig {
        base: stack_ptr as usize, //stack_ptr as usize
        size: size_of_val(&user_stack),
        range_type: RangeType::TOR,
        pmp_index: 2 as usize,
        permission: Permission::RW,
        locked: Lock::UNLOCKED,
    };

    pmp1.set();
    pmp2.set();
    /*
        let sp: usize;
        asm!("mv {}, sp",
            out(reg) sp);
        sprintln!("M-mode sp::{:0X}", sp);
    */
    //Setup mcounteren to allow u-mode access to cycle, instret, and time
    mcounteren::set_cy();
    mcounteren::set_ir();

    //Setup registers for user mode entry
    mepc::write(user_entry as usize); // Entry point for user mode
    mstatus::set_mpp(mstatus::MPP::User); //previous privilege set to user
    mstatus::clear_mie();
    mstatus::set_mpie();

    asm!(
            "mv ra, zero",
            "mv sp, {0}",
            "mret",
            in(reg) stack_ptr,
    );
}
