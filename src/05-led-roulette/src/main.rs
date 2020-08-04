#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let period = 50_u16;

    let mut led_1: usize = 0;
    let mut led_2: usize = 1;

    loop {
        leds[led_2].on();
        delay.delay_ms(period);
        leds[led_1].off();
        delay.delay_ms(period);

        led_1 = led_2;
        if led_2 == 7 {
            led_2 = 0
        } else {
            led_2 += 1;
        }
    }
}
