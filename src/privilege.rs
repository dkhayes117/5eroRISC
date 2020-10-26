use riscv::register::{mepc,mstatus};
use riscv::register::mtvec;
use hifive1::sprintln;
use crate::cpu::{StackFrame, STACK_SIZE};
use crate::trap::trap_handler;
use crate::pmp::{Pmpconfig,RangeType, Permission, Mlock};
//use crate::pmp::napot_range;


pub unsafe fn user_app_entry(user_entry:usize){
        //Create user stack and determine stack pointer and trap handler
        let mut user_stack = StackFrame{ stack: [0;STACK_SIZE] };
        let raw_ptr: *const StackFrame = &user_stack;
        let stack_ptr: *const StackFrame = raw_ptr.offset(1); //Top of stack

  /*      //Create stack frame for user mode
        let mut user_stack = StackFrame::new();
        let raw_ptr = &user_stack as *const StackFrame as *const();
        let stack_ptr = raw_ptr.offset(1);
*/
        sprintln!("bottom of stack::{:0X}", raw_ptr as usize);
        sprintln!("top of stack::{:0X}", stack_ptr as usize);

        //sprintln!("Trap Address::{:0X}",trap_address);
        sprintln!("User Entry::{:0X}",user_entry);

        let trap_address = trap_handler as *const();

        let pmp1 = Pmpconfig{
                base: raw_ptr as usize,
                size: STACK_SIZE,
                range_type: RangeType::OFF,
                pmp_index: 1 as usize,
                permission: Permission::RW,
                locked: Mlock::UNLOCKED
        };

        let pmp2 = Pmpconfig{
                base: 0x80003FFF,
                size: STACK_SIZE,
                range_type: RangeType::TOR,
                pmp_index: 2 as usize,
                permission: Permission::RW,
                locked: Mlock::UNLOCKED
        };

        pmp1.set();
        pmp2.set();

        //pmpaddr0::write(0x2040_0000); // All memory can be accessed
        //pmpaddr1::write(napot_range(raw_ptr as usize, STACK_SIZE)); // All of RAM is accessable
        //pmpcfg0::write(0x1B0F);

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



