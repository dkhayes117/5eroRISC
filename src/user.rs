// User Application

//use riscv::register::minstret;
//use crate::syscall::{syscall,SyscallType};


pub fn user_app(){

    //sprintln!("User Mode Entered!");  // Verify that user mode has been entered
    //let p1: usize = 22;
    //let p2: usize = 44;

    //minstret::read();   //Attempt to access so we know we returned to user mode

    //unsafe{syscall(SyscallType::PrintToConsole,p1, p2);}
    // Function to eliminate integers that are not prime

    // Function to eliminate integers that are not prime
    fn sieve(primes: &mut [u16], factor: u16) {
        for i in 0..primes.len() {
            let value = primes[i];
            if value != 0 && value != factor {
                if value % factor == 0 {
                    primes[i] = 0;
                }
            }
        }
    }

    let mut primes: [u16; 100] = [0; 100];
    for i in 2..=primes.len() - 1 {
        primes[i] = i as u16;
    }

    for i in 0..primes.len() {
        let factor = primes[i];
        if factor != 0 {
            sieve(&mut primes, factor);
        }
    }

/*
   // Force Stack Overflow
    unsafe{asm!(
    "addi sp, sp, 16",
    )}

    fn overflow() -> usize{
        let x = 1;
        x + 20
    }
    let _z = overflow();
 */
    //unsafe{syscall(SyscallType::PrintToConsole,p1, p2);}
    unsafe{asm!("ecall")};

    //minstret::read();   //Attempt to access so we know we returned to user mode



    loop{};
}