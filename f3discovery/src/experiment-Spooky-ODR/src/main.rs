#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux72::{ITM, entry, iprintln};

const GPIOE_ODR: u32 = 0x4800_1014;
const GPIOE_BSRR: u32 = 0x4800_1018;

fn iprint_odr(itm: &mut ITM) {
    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn start() -> ! {
    let (mut itm, _) = aux72::init();

    unsafe {
        // A magic addresses!
        const GPIOE_BSRR: u32 = 0x4800_1018;

        iprintln!(&mut itm.stim[0], "Initial contents of ODR");
        // Print the initial contents of ODR
        iprint_odr(&mut itm);

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprintln!(
            &mut itm.stim[0],
            "ODR after turning the North LED (No 9) on"
        );
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprintln!(
            &mut itm.stim[0],
            "ODR after turning the East LED (No 11) on"
        );
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprintln!(
            &mut itm.stim[0],
            "ODR after turning off the North LED (No 9)"
        );
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprintln!(
            &mut itm.stim[0],
            "ODR after turning off the East LED (No 11)"
        );
        iprint_odr(&mut itm);
    }

    loop {}
}
