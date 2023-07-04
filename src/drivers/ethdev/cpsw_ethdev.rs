use crate::drivers::ethdev::EthDev;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly,ReadWrite,WriteOnly},
 };

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (0x0 =>_reserverd1),
        (0x110 => VERSION: ReadWrite<u32>),
        (0x114 =>_reserverd2),  
        (0x118 => FR: ReadWrite<u32, FR::Register>),
        (0x11C => @END),
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

pub struct cpsw {
    numOfRxQue: u8,
    numOfTxQue: u8,
    register:&'static mut Registers,
}

impl cpsw {
    pub fn new(base_addr:usize, totalRxQue: u8, totalTxQue: u8)->Self {
        Self {
            register: unsafe { &mut *(base_addr as *mut Registers) },
            numOfRxQue:totalRxQue,
            numOfTxQue:totalTxQue,
        }
    }
}

impl EthDev for cpsw {
    fn init(&self) {
        
    }

    fn dev_start(&self) {
        let mut _cnt = 0;
        _cnt += 1;
    }

    fn dev_stop(&self) {
        
    }

    fn rx_queue_setup(&self) {
        
    }
    fn stats_get(&self) {
        
    }

    fn tx_queue_setup(&self) {
        
    }
}