use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::{anyhow, Result};
use display_interface_parallel_gpio::*;
use embedded_graphics::{
    mono_font::ascii::FONT_6X10,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use esp_idf_hal::{delay::Ets, gpio::*, peripherals::Peripherals};
use mipidsi::{models::ST7789, Builder, Display, Orientation};
use std::{thread, time::Duration};

// region:    --- Type Aliases
type LgBus<'a> = Generic8BitBus<
    PinDriver<'a, Gpio39, Output>,
    PinDriver<'a, Gpio40, Output>,
    PinDriver<'a, Gpio41, Output>,
    PinDriver<'a, Gpio42, Output>,
    PinDriver<'a, Gpio45, Output>,
    PinDriver<'a, Gpio46, Output>,
    PinDriver<'a, Gpio47, Output>,
    PinDriver<'a, Gpio48, Output>,
>;
type LgInterface<'a> =
    PGPIO8BitInterface<LgBus<'a>, PinDriver<'a, Gpio7, Output>, PinDriver<'a, Gpio8, Output>>;
type LgDisplayLifetime<'a> = Display<LgInterface<'a>, ST7789, PinDriver<'a, Gpio5, Output>>;
type LgDisplay = LgDisplayLifetime<'static>;
// endregion: --- Type Aliases

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let rst = PinDriver::output(peripherals.pins.gpio5)?;
    let dc = PinDriver::output(peripherals.pins.gpio7)?;
    let wr = PinDriver::output(peripherals.pins.gpio8)?;
    let mut rd = PinDriver::output(peripherals.pins.gpio9)?;

    let mut backlight = PinDriver::output(peripherals.pins.gpio38)?;
    let mut power = PinDriver::output(peripherals.pins.gpio15)?;
    let mut delay = Ets;

    power.set_high()?;
    rd.set_high()?;

    let bus: LgBus = Generic8BitBus::new((
        PinDriver::output(peripherals.pins.gpio39)?,
        PinDriver::output(peripherals.pins.gpio40)?,
        PinDriver::output(peripherals.pins.gpio41)?,
        PinDriver::output(peripherals.pins.gpio42)?,
        PinDriver::output(peripherals.pins.gpio45)?,
        PinDriver::output(peripherals.pins.gpio46)?,
        PinDriver::output(peripherals.pins.gpio47)?,
        PinDriver::output(peripherals.pins.gpio48)?,
    ))
    .expect("Unable to initialize 8bit bus");

    let di: LgInterface = PGPIO8BitInterface::new(bus, dc, wr);

    // create driver
    let mut display = Builder::st7789(di)
        .with_display_size(170, 320)
        .with_orientation(Orientation::Portrait(true))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, Some(rst))
        .unwrap();

    // turn on the backlight
    backlight.set_high()?;
    //let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
    //let ferris = Image::new(&raw_image_data, Point::new(0, 0));

    // draw image on red background
    //ferris.draw(&mut display).unwrap();
    display.clear(Rgb565::BLACK).unwrap();

    println!("Cleared display!");

    loop {
        draw_rectangle(&mut display, Rgb565::GREEN, 300)?;
        draw_rectangle(&mut display, Rgb565::BLUE, 250)?;
        draw_rectangle(&mut display, Rgb565::RED, 200)?;
        draw_rectangle(&mut display, Rgb565::MAGENTA, 150)?;
        draw_rectangle(&mut display, Rgb565::YELLOW, 100)?;
        draw_rectangle(&mut display, Rgb565::CYAN, 50)?;
        thread::sleep(Duration::from_secs(30));

        // set_display_color(&mut display, Rgb565::GREEN, "GREEN")?;
        // set_display_color(&mut display, Rgb565::RED, "RED")?;
        // set_display_color(&mut display, Rgb565::YELLOW, "YELLOW")?;
        // set_display_color(&mut display, Rgb565::BLUE, "BLUE")?;
    }
}

fn set_display_color(display: &mut LgDisplay, color: Rgb565, text: &str) -> Result<()> {
    println!("Draw color {text}");

    Rectangle::new(Point::new(50, 50), Size::new(200, 100))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
        .draw(display)
        .map_err(|_| anyhow!("unable to draw rectangle"))?;

    // Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    Text::new(text, Point::new(100, 50), style)
        .draw(display)
        .map_err(|_| anyhow!("unable to draw text"))?;

    thread::sleep(Duration::from_secs(10));

    Ok(())
}

fn draw_rectangle(display: &mut LgDisplay, color: Rgb565, size: u32) -> Result<()> {
    Rectangle::new(Point::new(0, 0), Size::new(size, size))
        .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
        .draw(display)
        .map_err(|_| anyhow!("unable to draw rectangle"))?;

    Ok(())
}
