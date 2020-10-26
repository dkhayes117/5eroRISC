use riscv::register::{pmpaddr0, pmpaddr1, pmpaddr2, pmpaddr3,
                      pmpaddr4, pmpaddr5, pmpaddr6, pmpaddr7,
                      pmpcfg0, pmpcfg1};
use hifive1::sprintln;

#[derive(Clone,Copy)]
pub enum Permission{
    NONE = 0,
    R = 1,
    W = 2,
    RW = 3,
    X = 4,
    RX = 5,
    WX = 6,
    RWX = 7
}

#[derive(Clone,Copy)]
pub enum RangeType{
    OFF = 0x0,
    TOR = 0x8,
    NA4 = 0x10,
    NAPOT = 0x18
}

#[derive(Clone,Copy)]
pub enum Mlock{
    UNLOCKED = 0x0,
    LOCKED = 0x80
}

#[derive(Clone,Copy)]
pub struct Pmpconfig{
    pub base: usize,
    pub size: usize,
    pub range_type: RangeType,
    pub pmp_index: usize,
    pub permission: Permission,
    pub locked: Mlock,
}

impl Pmpconfig {
    ///Find shift value
    fn shift_val(&self) -> usize{
        if self.pmp_index > 3{
            self.pmp_index - 4 * 8
        }
        else {
            self.pmp_index * 8
        }
    }

    ///Determines address value when not TOR
    fn address(&self) -> usize {
        match self.range_type{
            RangeType::TOR => {self.base >> 2}
            RangeType::OFF => {self.base >> 2}
            _ => {self.base + ((self.size / 2) - 1) >> 2}
        }

    }
    ///Write config and address values
    pub fn set(&self) {
        let shift = self.shift_val();
        let mask = 0xFF << shift;
        let cfg_value = (self.permission as usize | self.range_type as usize | self.locked as usize) << shift;
        let range = self.address();
        match self.pmp_index {
            0 => {pmpcfg0::write(cfg_value | (pmpcfg0::read() & !mask));
                  pmpaddr0::write(range)
            }
            1 => {pmpcfg0::write(cfg_value | (pmpcfg0::read() & !mask));
                pmpaddr1::write(range)
            }
            2 => {pmpcfg0::write(cfg_value | (pmpcfg0::read() & !mask));
                pmpaddr2::write(range)
            }
            3 => {pmpcfg0::write(cfg_value | (pmpcfg0::read() & !mask));
                pmpaddr3::write(range)
            }
            4 => {pmpcfg1::write(cfg_value | (pmpcfg1::read() & !mask));
                pmpaddr4::write(range)
            }
            5 => {pmpcfg1::write(cfg_value | (pmpcfg1::read() & !mask));
                pmpaddr5::write(range)
            }
            6 => {pmpcfg1::write(cfg_value | (pmpcfg1::read() & !mask));
                pmpaddr6::write(range)
            }
            7 => {
                pmpcfg1::write(cfg_value | (pmpcfg1::read() & !mask));
                pmpaddr7::write(range)
            }
            _ => {panic!()}

        }
    }

}

pub fn napot_range(base: usize, size: usize) -> usize {
    base + ((size / 2) - 1) >> 2
    }

// Read the pmp registers
pub fn print_pmp_addresses() {
    sprintln!("pmpaddr0: {:#X}", pmpaddr0::read());
    sprintln!("pmpaddr1: {:#X}", pmpaddr1::read());
    sprintln!("pmpaddr2: {:#X}", pmpaddr2::read());
    sprintln!("pmpaddr3: {:#X}", pmpaddr3::read());
    sprintln!("pmpaddr4: {:#X}", pmpaddr4::read());
    sprintln!("pmpaddr5: {:#X}", pmpaddr5::read());
    sprintln!("pmpaddr6: {:#X}", pmpaddr6::read());
    sprintln!("pmpaddr7: {:#X}\n", pmpaddr7::read());
}

pub fn print_register_by_byte(x: usize, mut i: i32) {
    sprintln!("         L  A-XWR");
    for byte in x.to_be_bytes().iter().rev() {
        sprintln!("pmp{}cfg: {:08b}", i, byte);
        i += 1;
    }
    sprintln!("\n");
}