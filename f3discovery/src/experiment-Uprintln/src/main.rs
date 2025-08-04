#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux111::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    }
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            // While there's ongoing transmission we wait
            while self.usart1.isr.read().txe().bit_is_clear() {}
            // Once the transmitter is clear, we write to the transmit register
            self.usart1.tdr.write(|w| w.tdr().bits(ch as u16));
        }

        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux111::init();

    let mut serial = SerialPort { usart1 };
    uprintln!(serial, "The answer is {}", 40 + 2);

    loop {}
}
