#![no_main]
#![no_std]

use core::fmt::{Write};

// Include assembly file during compilation.
// We need to include some things at the top of
// the text section.
use core::panic::PanicInfo;
use core::arch::global_asm;
use core::ptr::write_volatile;
use core::sync::atomic::AtomicBool;
mod drivers;
use crate::drivers::serial::uart::Uart;
use crate::drivers::serial::UartDev;
use drivers::ethdev::EthDev;
use crate::drivers::ethdev::cpsw_ethdev::cpsw; 
use crate::drivers::fifo::FIFO;

/*const LMEM_BASE_ADDR_WB: usize = 0x4000000;
const LMEM_BASE_ADDR_WT: usize = 0x4800000;
const LMEM_BASE_ADDR_U: usize  = 0x5000000;
const DLM_BASE_ADDR: usize    = 0x00080000;*/

const LMEM_DEBUG_OFFSET: usize = 0xF660;
const DLM_DEBUG_OFFSET: usize  = 0x400;
const DDR_OFFSET: usize  = 0x000;
const CPSW_OFFSET: usize  = 0x3C040000;
const UART_BASE_OFFSET: usize  = 0x80000000;

const CPSW_MAX_RX_RINGS:u8 = 1;
const CPSW_MAX_TX_RINGS:u8 = 1;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn _main() -> ! {
    
      
    /*let lmem_dbg_ptr = (LMEM_BASE_ADDR_U + LMEM_DEBUG_OFFSET) as *mut u32;
    let dlm_dbg_ptr  = (DLM_BASE_ADDR + DLM_DEBUG_OFFSET) as *mut u32;*/
    let mut serial = Uart::new(UART_BASE_OFFSET as usize);
    serial.config();
    writeln!(&mut serial, "Initialization... \n").unwrap();
    
    let eth = cpsw::new(CPSW_OFFSET as usize, 
    CPSW_MAX_RX_RINGS, CPSW_MAX_TX_RINGS);
    eth.init();
    
    //writeln!(&mut serial, "Initialization... {:?}\n", unsafe {ddr_ptr.read_volatile();}).unwrap();
    //unsafe {let mut ddr_base: *mut u32 = &mut __ddr_base;}
       
    loop{}
}

