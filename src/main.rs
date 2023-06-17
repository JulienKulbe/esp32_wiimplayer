use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use esp_idf_hal::{
    delay::Ets,
    gpio::{AnyIOPin, Gpio12, Gpio13, Gpio38, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9, PinDriver},
    interrupt::IntrFlags,
    spi::{config::Config, config::DriverConfig, Dma, SpiDeviceDriver, SpiDriver, SPI2},
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use mipidsi::Builder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let dc = PinDriver::input_output_od(unsafe { Gpio7::new() })?;
    let rst = PinDriver::input_output_od(unsafe { Gpio5::new() })?;
    let mut delay = Ets;

    let spi = unsafe { SPI2::new() };
    let sclk = unsafe { Gpio8::new() };
    let sdo = unsafe { Gpio9::new() };
    let cs = unsafe { Gpio6::new() };
    let bl = unsafe { Gpio38::new() };

    let config = DriverConfig::new();
    let config = config.dma(Dma::Channel1(320 * 170 * 2 + 8));
    //let config = config.intr_flags();

    let spi = SpiDriver::new(spi, sclk, sdo, None::<AnyIOPin>, &config)?;
    let spi = SpiDeviceDriver::new(spi, Some(cs), &Config::new())?;

    // create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SPIInterfaceNoCS::new(spi, dc);

    // create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::st7789(di)
        .init(&mut delay, Some(rst))
        .map_err(|_| Box::<dyn Error>::from("display init"))?;

    let mut bl = PinDriver::input_output_od(bl)?;
    bl.set_high()?;

    // clear the display to black
    display
        .clear(Rgb565::RED)
        .map_err(|_| Box::<dyn Error>::from("clear display"))?;

    Ok(())
}
