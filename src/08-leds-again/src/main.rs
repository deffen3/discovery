#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // initialize GPIOE
    //RCC: Reset and Clock Control
    //AHBENR: AHB peripheral clock enable register (RCC_AHBENR)
    /*
    Bit 21 IOPEEN: I/O port E clock enable
    Set and cleared by software.
    0: I/O port E clock disabled
    1: I/O port E clock enabled
    */
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

    // configure as output
    //GPIO port mode register (GPIOx_MODER)
    /*
    These bits are written by software to configure the I/O mode.
    00: Input mode (reset state)
    01: General purpose output mode
    10: Alternate function mode
    11: Analog mode
    */
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });

    // Turn on all the LEDs in the compass
    //GPIO port output data register (GPIOx_ODR)
    /*
    Bits 15:0 ODRy: Port output data bit (y = 0..15)
    These bits can be read and written by software.
    Note: For atomic bit set/reset, the ODR bits can be individually set and/or reset by writing to
    the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    */
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
