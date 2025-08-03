#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, mut itm) = aux11::init();

    // Send a single character
    usart1.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

    let now = mono_timer.now();
    // Send a string
    for ch in b"The quick brown fox jumps over the lazy dog." {
        while usart1.isr.read().txe().bit_is_clear() {}

        usart1.tdr.write(|w| w.tdr().bits(*ch as u16));
    }
    let elapsed = now.elapsed(); // in ticks

    iprintln!(
        &mut itm.stim[0],
        "your for loop took {} ticks and {} us",
        elapsed,
        elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}
