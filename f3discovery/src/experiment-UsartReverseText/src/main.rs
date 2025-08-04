#![no_main]
#![no_std]

use core::fmt::Write;

use aux114::entry;

#[macro_use]
mod uprintln;
use uprintln::*;

use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mt, _itm) = aux114::init();

    let mut serial = SerialPort { usart: usart1 };

    let mut buffer: Vec<u8, 64> = Vec::new();

    loop {
        buffer.clear();

        'inner: loop {
            // This isr guy never gets the rxne bit set when previously encountered a utf8 character.
            // Utf8 ends with an endless loop (or doesn't end because it's endless)
            while serial.usart.isr.read().rxne().bit_is_clear() {}

            let ch: u8 = serial.usart.rdr.read().bits() as u8;
            if ch != 13 {
                // Key code is ENTER
                if buffer.push(ch).is_ok() {
                    uprint!(serial, "{}", ch as char);
                } else {
                    uprintln!(
                        serial,
                        "Hey! We couldn't write {} to buffer! This is an ERROR! Exiting.",
                        ch
                    );
                    loop {}
                }
            } else {
                // Key code is not ENTER, so something else
                uprint!(serial, "\n\r");
                for ch in buffer.iter().rev() {
                    uprint!(serial, "{}", *ch as char);
                }
                uprint!(serial, "\n\r");
                break 'inner;
            }
        }
    }
}
