#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Set the timer to go off in `ms` ticks
    // 1 tick = 1 ms
    tim6.arr.write(|w| w.arr().bits(ms));
    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());
    // Wait until the alarm goes off (until the update event occurs)
    while !tim6.sr.read().uif().bit_is_set() {}
    // Clear the update event flag
    tim6.sr.modify(|_, w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // TODO initialize TIM6
    // Enable power to timer counter 6
    rcc.apb1enr.write(|w| w.tim6en().set_bit());
    // Timer 6 control register opm bit controls one pulse mode
    // We enable it for one pulse mode.
    // CEN bit controls counter enable.
    // We keep it disabled for now.
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure prescaler so that counter operates at 1 khz (1 millisecond period)
    // 0.001 seconds * 8_000_000 = 8000 ticks per millisecond
    // But starts from 0 so 7999
    tim6.psc.write(|w| w.psc().bits(7999));

    loop {
        let mut ms = 1024;
        for _ in 0..10 {
            // Gradually increase the speed of revolutions by 10 steps by diving by 2 (2^10 = 1024)
            ms /= 2;

            for curr in 0..8 {
                let next = (curr + 1) % 8;

                leds[next].on().unwrap();
                delay(tim6, ms);
                leds[curr].off().unwrap();
                delay(tim6, ms);
            }
        }
    }
}
