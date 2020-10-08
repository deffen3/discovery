#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction,
    lsm303dlhc_I16x3, l3gd20_I16x3, Sensitivity, Scale};
use m::Float;

#[entry]
fn main() -> ! {
    const MAG_XY_GAIN: f32 = 1100.; // LSB / G
    const MAG_Z_GAIN: f32 = 980.; // LSB / G    

    let (mut lsm303dlhc, mut l3gd20, mut delay, mut itm, gpioe) = aux15::init();
    //gpioe is only safe if we stick to using just bsrr?

    // Set Mode for LEDs
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


    // Range: [-16g, +16g]. Sensitivity ~ 12 g / (1 << 14) LSB
    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();
    const ACCEL_SENSITIVITY: f32 = 12. / (1 << 14) as f32;

    // Resolution: 12-bit
    // Range: [-40, +85]
    // from 16-bit
    // is 0 = +85, distance from TjMax?
    const TEMP_1_SCALE: f32 = -(85. - (-40.)) / 4096.; // ??
    const TEMP_1_OFFSET: f32 = 85.0; // ??

    // ??
    const TEMP_2_SCALE: f32 = -(85. - (-40.)) / 256.; // ??
    const TEMP_2_OFFSET: f32 = 85.0; // ??

    // 500 Degrees Per Second
    l3gd20.set_scale(Scale::Dps500).unwrap();
    const GYRO_GAIN: f32 = 65.536; // ??

    //Header Output
    iprintln!(&mut itm.stim[0],
        "mag_x,mag_y,mag_z,\
        accel_x,accel_y,accel_z,\
        gyro_x,gyro_y,gyro_z,\
        temp_1,temp_2", 
    );

    loop {
        let lsm303dlhc_I16x3 { x:mag_x, y:mag_y, z:mag_z } = lsm303dlhc.mag().unwrap();
        let lsm303dlhc_I16x3 { x:accel_x, y:accel_y, z:accel_z } = lsm303dlhc.accel().unwrap();
        let temp_1: i16 = lsm303dlhc.temp().unwrap();

        let l3gd20_I16x3 { x:gyro_x, y:gyro_y, z:gyro_z } = l3gd20.gyro().unwrap();
        let temp_2: i8 = l3gd20.temp().unwrap();

        let theta = (mag_y as f32).atan2(mag_x as f32); // in radians
         
        //turn off all LEDs
        gpioe.bsrr.write(|w| {
            w.br8().set_bit();
            w.br9().set_bit();
            w.br10().set_bit();
            w.br11().set_bit();
            w.br12().set_bit();
            w.br13().set_bit();
            w.br14().set_bit();
            w.br15().set_bit()
        });

        //turn on dir LED based on theta mag angle
        if theta < -7. * PI / 8. {
            //Direction::North = LD3
            gpioe.bsrr.write(|w| { w.bs9().set_bit() });
        } else if theta < -5. * PI / 8. {
            //Direction::Northwest = LD4
            gpioe.bsrr.write(|w| { w.bs8().set_bit() });
        } else if theta < -3. * PI / 8. {
            //Direction::West = LD6
            gpioe.bsrr.write(|w| { w.bs15().set_bit() });
        } else if theta < -PI / 8. {
            //Direction::Southwest = LD8
            gpioe.bsrr.write(|w| { w.bs14().set_bit() });
        } else if theta < PI / 8. {
            //Direction::South = LD10
            gpioe.bsrr.write(|w| { w.bs13().set_bit() });
        } else if theta < 3. * PI / 8. {
            //Direction::Southeast = LD9
            gpioe.bsrr.write(|w| { w.bs12().set_bit() });
        } else if theta < 5. * PI / 8. {
            //Direction::East = LD7
            gpioe.bsrr.write(|w| { w.bs11().set_bit() });
        } else if theta < 7. * PI / 8. {
            //Direction::Northeast = LD5
            gpioe.bsrr.write(|w| { w.bs10().set_bit() });
        } else {
            //Direction::North = LD3
            gpioe.bsrr.write(|w| { w.bs9().set_bit() });
        };

        let temp_1_C = TEMP_1_OFFSET + (temp_1 as f32 * TEMP_1_SCALE); //deg Celsius
        let temp_2_C = TEMP_2_OFFSET + (temp_2 as f32 * TEMP_2_SCALE);

        let mag_x_mG = f32::from(mag_x) / MAG_XY_GAIN; //mG = milliGauss, about 200-600mG depending on location
        let mag_y_mG = f32::from(mag_y) / MAG_XY_GAIN;
        let mag_z_mG = f32::from(mag_z) / MAG_Z_GAIN;

        let accel_x_g = f32::from(accel_x) * ACCEL_SENSITIVITY; //1g = 9.8m/s
        let accel_y_g = f32::from(accel_y) * ACCEL_SENSITIVITY;
        let accel_z_g = f32::from(accel_z) * ACCEL_SENSITIVITY;

        let gyro_x_dps = f32::from(gyro_x) / GYRO_GAIN; //dps = deg/s
        let gyro_y_dps = f32::from(gyro_y) / GYRO_GAIN;
        let gyro_z_dps = f32::from(gyro_z) / GYRO_GAIN;

        let mag_magnitude = (mag_x_mG * mag_x_mG +
            mag_y_mG * mag_y_mG +
            mag_z_mG * mag_z_mG).sqrt();

        let accel_magnitude = (accel_x_g * accel_x_g +
            accel_y_g * accel_y_g +
            accel_z_g * accel_z_g).sqrt();

        let gyro_magnitude = (gyro_x_dps * gyro_x_dps +
            gyro_y_dps * gyro_y_dps + 
            gyro_z_dps * gyro_z_dps).sqrt();

        //Raw Data Output
        iprintln!(&mut itm.stim[0],
            "{},{},{},{},{},{},{},{},{},{},{}", 
            mag_x, mag_y, mag_z,
            accel_x, accel_y, accel_z,
            gyro_x, gyro_y, gyro_z,
            temp_1, temp_2
        );

        delay.delay_ms(200_u8);
    }
}
