#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};
use embassy_rp::{
    self, 
    gpio::{Level, Output, OutputOpenDrain}, 
    i2c::Config,
};
use baro_test as bmp280_ehal;
use defmt::*;
use embedded_hal::i2c::I2c;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let p = embassy_rp::init(Default::default());

    let scl = p.PIN_15;
    let sda = p.PIN_14;

    let mut i2c_config = Config::default();
    i2c_config.frequency = 100_000;

    let mut led = Output::new(p.PIN_25, Level::Low);
    led.set_high();
    let mut i2c = embassy_rp::i2c::I2c::new_blocking(p.I2C1, scl, sda, i2c_config);

    for _ in 0..1_000_000 {
        cortex_m::asm::nop();
    }

    info!("writing to i2c");
    let addr = 0x76;

    // let mut id = [0u8; 1];
    // unwrap!(i2c.write(addr, &[0xD0]));
    // info!("first i2c write complete");
    // for _ in 0..1_000_000 {
    //     cortex_m::asm::nop();
    // }
    // unwrap!(i2c.read(addr, &mut id));
    // info!("i2c read complete");


    // --- Configure BMP280 ---
    // ctrl_meas = 0b00100111
    //   temp oversampling x1 (001), pressure x1 (001), normal mode (11)
    // i2c.write(addr, &[0xF4, 0b00100111]).unwrap();
    // info!("second i2c write complete");
    // // config = 0b10100000
    // //   standby 1000 ms (101), filter off (000)
    // i2c.write(addr, &[0xF5, 0b10100000]).unwrap();
    // info!("third i2c write complete");
    // // Small delay (~10ms) to let data be ready
    // for _ in 0..10_000 {
    //     cortex_m::asm::nop();
    // }

    
    
    // let mut temp_raw = [0u8; 3];
    // i2c.write_read(addr, &[0xFA], &mut temp_raw).unwrap();
    

    // info!("successfully wrote to I2C");

    // let mut bmp280 = match bmp280_ehal::BMP280::new(i2c) {
    //     Ok(bmp) => {
    //         defmt::info!("BMP280 initialized successfully");
    //         bmp
    //     },
    //     Err(e) => {
    //         defmt::error!("Failed to initialize BMP280: {:?}", e);
    //         loop {
    //             cortex_m::asm::delay(1_000_000);
    //             // defmt::error!("Error looping");
    //         }
    //     }
    // };

    loop {
        // let temperature = bmp280.temp();
        // let pressure = bmp280.pressure();
        // let altitude = bmp280.altitude();
        info!("Looping successfully");
        // defmt::info!("Temp: {} C", temperature);
        // defmt::info!("Pressure: {} Pa", pressure);
        // defmt::info!("Altitude: {} m", altitude);

        cortex_m::asm::delay(1_000_000);
    }
    
}
