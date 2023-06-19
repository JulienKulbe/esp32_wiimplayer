use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::thread;
use std::time::Duration;

use anyhow::Result;

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::units::FromValueType;

use display_interface_spi::SPIInterfaceNoCS;

use embedded_hal::spi::MODE_3;

use embedded_graphics::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, primitives::Rectangle};

use mipidsi::{Builder, Orientation};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let rst = PinDriver::output(peripherals.pins.gpio5)?;
    let dc = PinDriver::output(peripherals.pins.gpio7)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio38)?;
    let mut power = PinDriver::output(peripherals.pins.gpio15)?;
    let sclk = peripherals.pins.gpio12;
    let sda = peripherals.pins.gpio10;
    //let sdi = peripherals.pins.gpio8;
    let cs = peripherals.pins.gpio6;

    let mut delay = Ets;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = config::Config::new()
        .baudrate(26.MHz().into())
        .data_mode(MODE_3);

    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sda,
        None::<AnyIOPin>, //Some(sdi), //
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789(di)
        //.with_display_size(320, 240)
        // set default orientation
        //.with_orientation(Orientation::Portrait(false))
        // initialize
        .init(&mut delay, Some(rst))
        .unwrap();

    power.set_high()?;

    // turn on the backlight
    backlight.set_high()?;
    //let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    //let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on red background
    display.clear(Rgb565::RED).unwrap();
    //ferris.draw(&mut display).unwrap();

    println!("Image printed!");

    loop {
        display.clear(Rgb565::RED).unwrap();
        thread::sleep(Duration::from_millis(1000));
        display.clear(Rgb565::YELLOW).unwrap();
        thread::sleep(Duration::from_millis(1000));
        display.clear(Rgb565::BLUE).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
