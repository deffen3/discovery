#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = aux14::init();

    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.sadd1().bits(MAGNETOMETER); //secondary's address
            w.rd_wrn().clear_bit(); //primary requests a write transfer
            w.nbytes().bits(1); //primary requests 1 byte
            w.start().set_bit(); //primary sets START to start generating message
            w.autoend().clear_bit() //primary sets AUTOEND off = SW end mode, to require a STOP condition from SW
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}
        //ISR = Interrupt and Status Register
        //TXIS = Transmit Interrupt Status, cleared by HW

        // Send the address of the register that we want to read: IRA_REG_M
        i2c1.txdr.write(|w| w.txdata().bits(IRA_REG_M));
        //TXDR = Transmit Data Register

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
        //TC = Transfer Complete
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.nbytes().bits(1); //primary requests a write transfer
            w.rd_wrn().set_bit(); //primary requests 1 byte
            w.start().set_bit(); //primary sets START to start generating message
            w.autoend().set_bit() //primary sets AUTOEND on = automatic HW end mode
        });

        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}
        //RXNE = Receive Data Register Not Empty, cleared by HW

        // Receive the contents of the register
        i2c1.rxdr.read().rxdata().bits()
        //RXDR = Receive Data Register
    };

    // Expected output: 0x0A - 0b01001000
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
