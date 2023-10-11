#![no_main]
#![no_std]
#![feature(exclusive_range_pattern)]

use adafruit_7segment::{Index, SevenSegment};
use ht16k33::{Dimming, Display, HT16K33};
use ism330dhcx::ctrl1xl::Odr_Xl;
use ism330dhcx::Ism330Dhcx;
use nucleo::hal::delay::Delay;
use nucleo::hal::prelude::*;
use nucleo_h7xx as nucleo;

const DISP_I2C_ADDR: u8 = 0x70;

#[cortex_m_rt::entry]
fn main() -> ! {
    // - board setup ----------------------------------------------------------

    let board = nucleo::board::Board::take().unwrap();

    let dp = nucleo::pac::Peripherals::take().unwrap();

    let core = nucleo::pac::CorePeripherals::take().unwrap();

    let ccdr = board.freeze_clocks(dp.PWR.constrain(), dp.RCC.constrain(), &dp.SYSCFG);

    let mut delay = Delay::new(core.SYST, ccdr.clocks);

    let pins = board.split_gpios(
        dp.GPIOA.split(ccdr.peripheral.GPIOA),
        dp.GPIOB.split(ccdr.peripheral.GPIOB),
        dp.GPIOC.split(ccdr.peripheral.GPIOC),
        dp.GPIOD.split(ccdr.peripheral.GPIOD),
        dp.GPIOE.split(ccdr.peripheral.GPIOE),
        dp.GPIOF.split(ccdr.peripheral.GPIOF),
        dp.GPIOG.split(ccdr.peripheral.GPIOG),
    );

    // Configure the SCL and the SDA pin for sensor I2C bus
    let scl = pins.d15.into_alternate_open_drain::<4>();
    let sda = pins.d14.into_alternate_open_drain::<4>();

    let mut i2c1 = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

    let mut sensor = Ism330Dhcx::new(&mut i2c1).unwrap();

    sensor
        .ctrl1xl
        .set_accelerometer_data_rate(&mut i2c1, Odr_Xl::Hz52)
        .expect("Don't know why setting data rate could fail");

    // Configure the SCL and the SDA pin for display I2C bus
    let scl = pins.d69.into_alternate_open_drain();
    let sda = pins.d68.into_alternate_open_drain();

    let i2c4 = dp
        .I2C4
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C4, &ccdr.clocks);

    let mut ht16k33 = HT16K33::new(i2c4, DISP_I2C_ADDR);
    ht16k33.initialize().expect("Failed to initialize ht16k33");
    ht16k33
        .set_display(Display::ON)
        .expect("Could not turn on the display!");
    ht16k33
        .set_dimming(Dimming::BRIGHTNESS_MAX)
        .expect("Could not set dimming!");

    loop {
        let temp = sensor.get_temperature(&mut i2c1).unwrap();
        // Formatting a float using the whole display

        if temp < -9.99 {
            ht16k33
                .update_buffer_with_float(Index::One, -9.99, 2, 10)
                .unwrap()
        } else if temp < 0.0 {
            ht16k33
                .update_buffer_with_float(Index::One, temp, 2, 10)
                .unwrap()
        } else if temp < 10.0 {
            ht16k33
                .update_buffer_with_float(Index::Two, temp, 2, 10)
                .unwrap();
            ht16k33.update_buffer_with_digit(Index::One, 0)
        } else if temp < 100.0 {
            ht16k33
                .update_buffer_with_float(Index::One, temp, 2, 10)
                .unwrap()
        } else {
            ht16k33
                .update_buffer_with_float(Index::One, 99.99, 2, 10)
                .unwrap()
        }

        'retries: for _ in 0..5 {
            match ht16k33.write_display_buffer() {
                Ok(_) => break 'retries,
                Err(e) => defmt::debug!("{:?}", e),
            }
            defmt::debug!("Retrying write_display_buffer");
        }

        delay.delay_ms(100u32);
    }

    nucleo_sensors::exit()
}
