#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};
// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = aux15::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians

        iprintln!(&mut itm.stim[0], "theta: {}", theta);

        // FIXME pick a direction to point to based on `theta`
        let led_angle = (2.0 * PI) / 8.0;
        let offset = led_angle / 2.0;
        let dir = match theta {
            x if x < offset && x > -offset => Direction::South,
            x if x < (led_angle + offset) && x > (led_angle - offset) => Direction::Southeast,
            x if x < ((2.0 * led_angle) + offset) && x > ((2.0 * led_angle) - offset) => {
                Direction::East
            }
            x if x < ((3.0 * led_angle) + offset) && x > ((3.0 * led_angle) - offset) => {
                Direction::Northeast
            }
            x if x < ((4.0 * led_angle) + offset) && x > ((4.0 * led_angle) - offset) => {
                Direction::North
            }
            // overflow
            x if x < ((-4.0 * led_angle) + offset) && x > ((-4.0 * led_angle) - offset) => {
                Direction::North
            }
            x if x < ((-3.0 * led_angle) + offset) && x > ((-3.0 * led_angle) - offset) => {
                Direction::Northwest
            }
            x if x < ((-2.0 * led_angle) + offset) && x > ((-2.0 * led_angle) - offset) => {
                Direction::West
            }
            x if x < ((-1.0 * led_angle) + offset) && x > ((-1.0 * led_angle) - offset) => {
                Direction::Southwest
            }
            _ => Direction::North,
        };

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(100_u8);
    }
}
