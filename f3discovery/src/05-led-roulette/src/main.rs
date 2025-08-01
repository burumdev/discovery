#![deny(unsafe_code)]
#![no_main]
#![no_std]

const LED_DURATION: u16 = 5;

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(LED_DURATION);
            leds[curr].off().ok();
            delay.delay_ms(LED_DURATION);
        }
    }
}
