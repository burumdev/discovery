#![no_main]
#![no_std]

use core::fmt::Write;

use aux114::entry;

#[macro_use]
mod uprintln;
use uprintln::*;

use heapless::Vec;

fn reverse_text(serial: &mut SerialPort, buffer: &Vec<u8, 64>) {
    uprint!(serial, "\n\r");
    for ch in buffer.iter().rev() {
        uprint!(serial, "{}", *ch as char);
    }
    uprint!(serial, "\n\r");
}

#[entry]
fn main() -> ! {
    let (usart1, _mt, _itm) = aux114::init();

    let mut serial = SerialPort { usart: usart1 };

    let mut buffer: Vec<u8, 64> = Vec::new();

    uprintln!(
        serial,
        "Uprintln and friends at your service. Type something calmly and press ENTER to have stm32 reverse it. I said CALMLY!\n\r* This crashes if you type so fast..\n\r* Or you type evil utf8 characters..\n\r",
    );

    loop {
        buffer.clear();

        'inner: loop {
            // This isr guy never gets the rxne bit set when previously encountered a utf8 character.
            // Utf8 ends with an endless loop (or doesn't end because it's endless)
            while serial.usart.isr.read().rxne().bit_is_clear() {}

            let ch: u8 = serial.usart.rdr.read().bits() as u8;
            if ch != 13 {
                // Keycode is NOT Enter
                if buffer.push(ch).is_ok() {
                    uprint!(serial, "{}", ch as char);
                } else {
                    uprintln!(
                        serial,
                        "\n\rOoops! We couldn't write '{}' to buffer. Buffer might be full. So let's interpret it as Enter key pressed.",
                        ch as char
                    );
                    // We execute the reverse operation as
                    // if user pressed enter.
                    reverse_text(&mut serial, &buffer);
                    // Restore the intended character to a clear buffer
                    // as a workaround of buffer overflow situation.
                    uprint!(serial, "{}", ch as char);
                    buffer.clear();
                    buffer.push(ch).unwrap();

                    continue 'inner;
                }
            } else {
                // Key code is ENTER
                reverse_text(&mut serial, &buffer);
                break 'inner;
            }
        }
    }
}
