// An example that uses timer 6
// to display 0 to 255 bytes in binary format
// with Leds configured as North being the Most Significant Bit
//
#![no_main]
#![no_std]

use aux91::entry;
use aux91::init;
use aux91::switch_hal::OutputSwitch;
use aux91::tim6::RegisterBlock as Tim6RegisterBlock;

// Busy wait delay function
// An interrupt should be used instead for effectiveness
// But for now, we'll just use this instead.
fn delay(tim6: &Tim6RegisterBlock, ms: u16) {
    // Auto reload register arr holds the auto reload value
    // or "clear timer on compare match" value in AVR terminology
    // 1 ms = 1 tick as configured in main function
    tim6.arr.write(|w| w.arr().bits(ms));
    // We enable the timer at this point with the CEN
    // counter enable bit in control register 1
    tim6.cr1.write(|w| w.cen().set_bit());

    // Status register sr of timer6 uif bit
    // or update interrupt flag will read 1 when
    // We match the auto reload value
    // So we "busy wait" till that happens.
    // We should use a hardware interrupt instead but
    // I don't know how to use them. Yet...
    while tim6.sr.read().uif().bit_is_clear() {}

    // We have to clear the uif flag manually..
    // STM32 can be a PITA sometimes.
    // There must be a simpler way of doing these things.
    tim6.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = init();
    let mut leds = leds.into_array();
    // Rearrange leds so North led is the most significant bit led.
    leds.reverse();

    // ---> Timer Setup
    // Enable timer 6
    rcc.apb1enr.write(|w| w.tim6en().set_bit());
    // Configure timer in one pulse or one shot mode, but disable the timing for now.
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // For a 1 millisecond per tick timing, we set the prescaler in 8000 ticks
    // Because 0.001 seconds * 8_000_000 hertz (CPU clock) = 8000
    // When starting point of 0 is taken into account we write 7999 to the prescaler register.
    tim6.psc.write(|w| w.psc().bits(7999));

    // We'll loop through byte values in half a second intervals.
    let ms = 500;
    loop {
        for byte in 0..255 {
            // We compare byte's bits starting from the least significant bit.
            for bit in (0..8).rev() {
                if byte & (1 << bit) > 0 {
                    leds[bit].on().unwrap();
                } else {
                    leds[bit].off().unwrap();
                }
            }
            delay(&tim6, ms);
        }
    }
}
