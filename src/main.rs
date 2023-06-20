use anyhow::anyhow;
use anyhow::Result;
use display_interface_parallel_gpio::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::{
    mono_font::ascii::FONT_6X10,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use mipidsi::{Builder, Orientation};
use std::thread;
use std::time::Duration;

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

    let bus = Generic8BitBus::new((
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

    let di = PGPIO8BitInterface::new(bus, dc, wr);

    // create driver
    let mut display = Builder::st7789(di)
        .with_display_size(320, 170)
        .with_orientation(Orientation::Landscape(true))
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
        set_display_color(&mut display, Rgb565::GREEN, "GREEN")?;
        set_display_color(&mut display, Rgb565::RED, "RED")?;
        set_display_color(&mut display, Rgb565::YELLOW, "YELLOW")?;
        set_display_color(&mut display, Rgb565::BLUE, "BLUE")?;
    }
}

fn set_display_color<DT>(display: &mut DT, color: Rgb565, text: &str) -> Result<()>
where
    DT: DrawTarget<Color = Rgb565>,
{
    println!("Draw color {text}");

    Rectangle::new(Point::new(0, 0), Size::new(200, 100))
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
