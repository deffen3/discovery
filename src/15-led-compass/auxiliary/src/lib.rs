//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude, stm32f30x::i2c1},
    led::{Direction, Leds},
    lsm303dlhc::{I16x3 as lsm303dlhc_I16x3, Sensitivity},
    l3gd20::{I16x3 as l3gd20_I16x3, Scale},
};

use f3::{
    hal::{i2c::I2c, spi::Spi, prelude::*, stm32f30x},
    Lsm303dlhc, l3gd20, L3gd20,
};

//pub fn init() -> (Leds, Lsm303dlhc, L3gd20, Delay, ITM) {
pub fn init() -> (Lsm303dlhc, L3gd20, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();

    // The `L3gd20` abstraction exposed by the `f3` crate requires a specific pin configuration to
    // be used and won't accept any configuration other than the one used here. Trying to use a
    // different pin configuration will result in a compiler error.
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
 
    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );


    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let l3gd20 = L3gd20::new(spi, nss).unwrap();

    let delay = Delay::new(cp.SYST, clocks);


    //let leds = Leds::new(gpioe);

    //(leds, lsm303dlhc, l3gd20, delay, cp.ITM)
    (lsm303dlhc, l3gd20, delay, cp.ITM)
}
