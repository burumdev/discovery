// Read bytes from the stm32f3 and print them to the com port
// Use minicom for it.
// Warning! Crashes when utf8 character is hit.
#![no_main]
#![no_std]

use core::fmt::Write;

#[macro_use]
mod uprintln;
use uprintln::*;

use aux113::entry;

#[entry]
fn main() -> ! {
    let (usart1, _mt, _itm) = aux113::init();
    let mut serial = SerialPort { usart: usart1 };

    loop {
        // This isr guy never gets the rxne bit set when previously encountered a utf8 character.
        // Utf8 ends with an endless loop (or doesn't end because it's endless)
        while serial.usart.isr.read().rxne().bit_is_clear() {}

        let char: Result<char, _> = serial.usart.rdr.read().bits().try_into();
        if let Ok(ch) = char {
            uprint!(serial, "{}", ch);
        }
    }
}
