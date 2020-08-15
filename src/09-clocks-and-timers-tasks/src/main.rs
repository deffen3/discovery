#![no_main]
#![no_std]

use aux9_tasks::entry;

#[entry]
fn main() -> ! {
    let (mut leds, rcc, tim6) = aux9_tasks::init();

    // Power on the TIM6 timer
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    // SR, the status register.
    // EGR, the event generation register.
    // CNT, the counter register.
    // PSC, the prescaler register.
    // ARR, the autoreload register.

    // CR1 Control Register 1
    // OPM Select one pulse mode
    // CEN Counter Enable - Keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Configure the prescaler to have the counter operate at 1 KHz
    // PSC Pre-scaler
    // Remember that the frequency of the counter is apb1 / (psc + 1) and that apb1 is 8 MHz.
    // APB1_CLOCK = 8 MHz
    // 8 MHz / (7999 + 1) = 1 KHz
    // The counter (CNT) will increase on every millisecond
    tim6.psc.write(|w| w.psc().bits(7999));

    let ms = 100;

    let mut seq_idx = 0;

    let led_tasks: [i32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

    loop {
        // Set the timer to go off in `ms` ticks
        // 1 tick = 1 ms
        tim6.arr.write(|w| w.arr().bits(ms));

        // CEN: Enable the counter
        tim6.cr1.modify(|_, w| w.cen().set_bit());

        for led_idx in 0..8 {
            if ((seq_idx / led_tasks[led_idx]) % 2) == 0 {
                leds[led_idx].on();
            } else {
                leds[led_idx].off();
            }
        }

        // Update LED sequence index
        seq_idx = seq_idx + 1;
        if seq_idx == led_tasks[7] * 2 {
            seq_idx = 0;
        }

        // Wait until the alarm goes off (until the update event occurs)
        // SR, Status Register
        // UIF, Update Interrupt Flag
        while !tim6.sr.read().uif().bit_is_set() {}

        // Clear the update event flag
        tim6.sr.modify(|_, w| w.uif().clear_bit());
    }
}
