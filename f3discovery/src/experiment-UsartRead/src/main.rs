#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux112::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux112::init();

    loop {
        // We busy wait for the interrupt-status register of usart register block
        // RXNE bit Receive data not empty bit to go HIGH
        // Which means there's new data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // RDR register of usart register block holds the received data bits
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        aux112::bkpt();
    }
}
