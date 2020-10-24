use riscv::register::{pmpaddr0, pmpaddr1, pmpaddr2, pmpaddr3,
                      pmpaddr4, pmpaddr5, pmpaddr6, pmpaddr7,
                      pmpcfg0, pmpcfg1};

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
pub struct Pmpconfig{
    pub base: usize,
    pub size: usize,
    pub range_type: RangeType,
    pub pmp_index: usize,
    pub permission: Permission,
    pub locked: bool,
}

impl Pmpconfig {
    ///Determines address value when not TOR
    pub fn address(&self) -> usize {
        match self.range_type{
            RangeType::TOR => {self.base}
            _ => {self.base + ((self.size / 2) - 1) >> 2}
        }

    }
    ///Create pmpconfig[x] value based on self
    pub fn cfg_value(&self) -> usize{
        let shift = self.pmp_index * 8;
        let lock = (self.locked as usize) << (shift + 8 as usize);
        self.permission as usize | self.range_type as usize | lock as usize
    }
    ///Write config and address values
    pub fn set(&self) {
        let cfg_value = self.cfg_value();
        let range = self.address();
        match self.pmp_index {
            0 => {pmpcfg0::write(cfg_value | pmpcfg0::read());
                  pmpaddr0::write(range)
            }
            1 => {pmpcfg0::write(cfg_value | pmpcfg0::read());
                pmpaddr1::write(range)
            }
            2 => {pmpcfg0::write(cfg_value | pmpcfg0::read());
                pmpaddr2::write(range)
            }
            3 => {pmpcfg0::write(cfg_value | pmpcfg0::read());
                pmpaddr3::write(range)
            }
            4 => {pmpcfg1::write(cfg_value | pmpcfg1::read());
                pmpaddr4::write(range)
            }
            5 => {pmpcfg1::write(cfg_value | pmpcfg1::read());
                pmpaddr5::write(range)
            }
            6 => {pmpcfg1::write(cfg_value | pmpcfg1::read());
                pmpaddr6::write(range)
            }
            7 => {
                pmpcfg1::write(cfg_value | pmpcfg1::read());
                pmpaddr7::write(range)
            }
            _ => {panic!()}

        }
    }
}
