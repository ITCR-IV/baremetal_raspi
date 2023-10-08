#![no_main]
#![no_std]

use ism330dhcx::ctrl1xl::Odr_Xl;
use ism330dhcx::Ism330Dhcx;
use nucleo::hal::delay::Delay;
use nucleo::hal::prelude::*;
use nucleo::led::Led;
use nucleo_h7xx as nucleo;

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

    // Configure the SCL and the SDA pin for our I2C bus
    let scl = pins.d15.into_alternate_open_drain::<4>();
    let sda = pins.d14.into_alternate_open_drain::<4>();

    let mut i2c = dp
        .I2C1
        .i2c((scl, sda), 100.kHz(), ccdr.peripheral.I2C1, &ccdr.clocks);

    let mut sensor = Ism330Dhcx::new(&mut i2c).unwrap();

    loop {
        sensor
            .ctrl1xl
            .set_accelerometer_data_rate(&mut i2c, Odr_Xl::Hz52)
            .expect("Don't know why setting data rate could fail");

        let temp = sensor.get_temperature(&mut i2c).unwrap();

        defmt::info!("Temperature: {}", temp);

        delay.delay_ms(1000u32);
    }

    nucleo_sensors::exit()
}
