use riscv::register::{pmpaddr0,pmpcfg0};
pub fn pmp_config(){
    let permissions: usize = 0xF;       //TOR alignment with RWX permissions
    pmpaddr0::write(0x20400000);   // All flash memory available
    pmpcfg0::write(permissions);
}
pub fn pmp_address(){}
