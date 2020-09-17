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

    //let (mut leds, mut lsm303dlhc, mut l3gd20, mut delay, mut itm) = aux15::init();
    let (mut lsm303dlhc, mut l3gd20, mut delay, mut itm) = aux15::init();

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



    loop {
        let lsm303dlhc_I16x3 { x:mag_x, y:mag_y, z:mag_z } = lsm303dlhc.mag().unwrap();
        let lsm303dlhc_I16x3 { x:accel_x, y:accel_y, z:accel_z } = lsm303dlhc.accel().unwrap();
        let temp_1: i16 = lsm303dlhc.temp().unwrap();

        let l3gd20_I16x3 { x:gyro_x, y:gyro_y, z:gyro_z } = l3gd20.gyro().unwrap();
        let temp_2: i8 = l3gd20.temp().unwrap();

        // let theta = (mag_y as f32).atan2(mag_x as f32); // in radians
        
        // let dir = if theta < -7. * PI / 8. {
        //     Direction::North
        // } else if theta < -5. * PI / 8. {
        //     Direction::Northwest
        // } else if theta < -3. * PI / 8. {
        //     Direction::West
        // } else if theta < -PI / 8. {
        //     Direction::Southwest
        // } else if theta < PI / 8. {
        //     Direction::South
        // } else if theta < 3. * PI / 8. {
        //     Direction::Southeast
        // } else if theta < 5. * PI / 8. {
        //     Direction::East
        // } else if theta < 7. * PI / 8. {
        //     Direction::Northeast
        // } else {
        //     Direction::North
        // };
        
        // leds.iter_mut().for_each(|led| led.off());
        // leds[dir].on();

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

        iprintln!(&mut itm.stim[0], "{} mG", mag_magnitude * 1_000.);
        iprintln!(&mut itm.stim[0], "{} {} {} mG", mag_x_mG, mag_y_mG, mag_z_mG);

        iprintln!(&mut itm.stim[0], "{} g", accel_magnitude);
        iprintln!(&mut itm.stim[0], "{} {} {} g", accel_x_g, accel_y_g, accel_z_g);

        iprintln!(&mut itm.stim[0], "{} deg/s", gyro_magnitude);
        iprintln!(&mut itm.stim[0], "{} {} {} deg/s", gyro_x_dps, gyro_y_dps, gyro_z_dps);

        iprintln!(&mut itm.stim[0], "{} {} degC", temp_1_C, temp_2_C);

        delay.delay_ms(100_u8);
    }
}
