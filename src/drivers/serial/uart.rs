
use crate::drivers::serial::UartDev;
use crate::drivers::fifo::FIFO;

const FIFO_BASE_ADDR: usize    = 0x8000_1000;

use core::fmt;
 use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly,ReadWrite,WriteOnly},
};

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (00 => DR: ReadWrite<u32>),
        (04 =>_reserverd1),  
        (0x18 => FR: ReadWrite<u32, FR::Register>),
        (0x1C => @END),
    }
}

register_bitfields! {
    u32,
    /// Flag Register.
    FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],

        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is full.
        /// - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],

        /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains
        /// set until the complete byte, including all the stop bits, has been sent from the shift
        /// register.
        ///
        /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether
        /// the UART is enabled or not.
        BUSY OFFSET(3) NUMBITS(1) []
    ],

}
        
pub struct Uart {
    register:&'static mut Registers,
    fifo: FIFO,
}

impl Uart {
    pub fn new(base_addr:usize)->Self { 
        Self {
            register: unsafe { &mut *(base_addr as *mut Registers) },
            fifo: FIFO::new(FIFO_BASE_ADDR as usize),
        }
    }
}
impl UartDev for Uart {
    
    fn config(&self) {
        self.register.FR.write(FR::TXFF.val(0));
        self.register.FR.write(FR::RXFE.val(1)); 
    }

    fn read_char(&self)->char {
        while self.register.FR.matches_all(FR::RXFE::SET) {
            ;
        }
        self.register.DR.get() as u8 as char
    }
    
    fn write_char(&mut self, c:char) {
        while self.register.FR.matches_all(FR::TXFF::SET) {
            ;
        }
        self.fifo.push(c);
        //self.register.DR.set(c as u32);
    }

    fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

impl fmt::Write for Uart {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
