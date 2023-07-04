const FIFO_SZ:u32 = 1024;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly,ReadWrite,WriteOnly},
};

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => SIZE: ReadWrite<u32>),
        (0x04 => WR_IDX: ReadWrite<u32>),  
        (0x08 => RD_IDX: ReadWrite<u32>),
        (0x0C => BUFF: [ReadWrite<u32>; 253]),
        (1024=> @END),
    }
}

/*extern "C" {
    
    // These symbols come from `linker.ld`
    static mut __ddr_global_pointer: [u32;FIFO_SZ as usize]; // Start of .bss section
}*/

pub struct FIFO {
    register:&'static mut Registers,
}

impl FIFO {
    pub fn new(base_addr:usize)->Self { 
        Self {
            register: unsafe { &mut *(base_addr as *mut Registers) },
        }
    }

    pub fn is_full (&self)-> bool {
        self.register.SIZE.get() == FIFO_SZ as u32
    }

    pub fn push (&mut self, val:char) -> bool {
        if self.is_full() == true {
            true
        }
        else {
            
            self.register.BUFF[self.register.WR_IDX.get() as usize].set(val as u32);
            self.register.WR_IDX.set(self.inc_idx(self.register.WR_IDX.get()));
            self.register.SIZE.set(self.register.SIZE.get() + 1);
            false
        }
    }
    pub fn pop (&mut self)-> Option<u8> {
        if self.register.SIZE.get() == 0
        {
            None
        }
        else {
            self.register.RD_IDX.set(self.inc_idx(self.register.RD_IDX.get()));
            self.register.SIZE.set(self.register.SIZE.get() -  1);
            Some(self.register.BUFF[self.register.WR_IDX.get() as usize].get() as u8)
        }
    }
    pub fn inc_idx(&self, idx:u32) ->u32{
        (idx + 1) % FIFO_SZ
    }

}