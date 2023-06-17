use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use mipidsi::Builder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let spi = todo!();
    let dc = todo!();
    let delay = todo!();
    let rst = todo!();

    // create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SPIInterfaceNoCS::new(spi, dc);
    // create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::st7789(di)
        .init(&mut delay, Some(rst))
        .map_err(|_| Box::<dyn Error>::from("display init"))?;

    // clear the display to black
    display
        .clear(Rgb565::RED)
        .map_err(|_| Box::<dyn Error>::from("clear display"))?;

    Ok(())
}
