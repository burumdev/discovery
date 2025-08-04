use core::fmt;

use aux113::usart1;

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

pub struct SerialPort {
    pub usart: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            // While there's ongoing transmission we wait
            while self.usart.isr.read().txe().bit_is_clear() {}
            // Once the transmitter is clear, we write to the transmit register
            self.usart.tdr.write(|w| w.tdr().bits(ch as u16));
        }

        Ok(())
    }
}
