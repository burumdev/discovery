#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux71::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let (mut itm, _) = aux71::init();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    unsafe {
        // Try to read from a non-existent register
        // Should result in a HardFault CPU exception from the stm32f3
        // Hard fault can be observed in GDB like this:
        // Breakpoint 3, cortex_m_rt::HardFault_ (ef=0x20009f60) at src/lib.rs:560
        let i_can_read = ptr::read_volatile(0x4800_1800 as *const u32);

        // This should not print as the line before should throw the CPU exception
        // (Blue Screen Of Detriment)
        iprintln!(
            &mut itm.stim[0],
            "As I said before.. I can read anything! Wait.. {}",
            i_can_read
        );
    }

    loop {}
}
