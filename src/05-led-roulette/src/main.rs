#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let step = 50_u16; //ms

    //time step when led[0] is turned on, shifted left back in time
    let on_shift = 2;
    //off is shifted left by -1, but we can't go negative, so -1 + 16 (with remainder math works out fine)
    let off_shift = 15;

    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    loop {
        for i in 0..16 {
            delay.delay_ms(step);

            let led_on_idx: usize = ((i + on_shift) / 2) % 8;

            leds[led_on_idx].on();

            let led_off_idx: usize = ((i + off_shift) / 2) % 8;

            leds[led_off_idx].off();
        }
    }
}
