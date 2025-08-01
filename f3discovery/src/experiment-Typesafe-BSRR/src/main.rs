#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux73::{ITM, RegisterBlock, entry, iprintln};

#[entry]
fn main() -> ! {
    let (mut itm, gpioe) = aux73::init();
    let itm_w = &mut itm.stim[0];

    iprintln!(itm_w, "Lighting up the North LED");
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    iprintln!(itm_w, "Lighting up the East LED");
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    iprintln!(itm_w, "Turn off the North LED");
    gpioe.bsrr.write(|w| w.br9().set_bit());

    iprintln!(itm_w, "Turn off the East LED");
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
