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
use esp_idf_hal::{delay::Ets, gpio::*};
use log::info;
use mipidsi::{models::ST7789, Builder, Display, Orientation};

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

pub struct TftPins {
    pub rst: Gpio5,
    pub dc: Gpio7,
    pub wr: Gpio8,
    pub rd: Gpio9,
    pub bl: Gpio38,
    pub pw: Gpio15,
    pub d0: Gpio39,
    pub d1: Gpio40,
    pub d2: Gpio41,
    pub d3: Gpio42,
    pub d4: Gpio45,
    pub d5: Gpio46,
    pub d6: Gpio47,
    pub d7: Gpio48,
}

pub struct TftDisplay {
    display: LgDisplay,
    rd: PinDriver<'static, Gpio9, Output>,
}

impl TftDisplay {
    pub fn new(pins: TftPins) -> Self {
        let rst = PinDriver::output(pins.rst).unwrap();
        let dc = PinDriver::output(pins.dc).unwrap();
        let wr = PinDriver::output(pins.wr).unwrap();
        let mut rd = PinDriver::output(pins.rd).unwrap();
        let mut backlight = PinDriver::output(pins.bl).unwrap();
        let mut power = PinDriver::output(pins.pw).unwrap();
        let mut delay = Ets;

        power.set_high().unwrap();
        rd.set_high().unwrap();

        let bus: LgBus = Generic8BitBus::new((
            PinDriver::output(pins.d0).unwrap(),
            PinDriver::output(pins.d1).unwrap(),
            PinDriver::output(pins.d2).unwrap(),
            PinDriver::output(pins.d3).unwrap(),
            PinDriver::output(pins.d4).unwrap(),
            PinDriver::output(pins.d5).unwrap(),
            PinDriver::output(pins.d6).unwrap(),
            PinDriver::output(pins.d7).unwrap(),
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
        backlight.set_high().unwrap();
        //let raw_image_data = ImageRawLE::new(include_bytes!("../examples/assets/ferris.raw"), 86);
        //let ferris = Image::new(&raw_image_data, Point::new(0, 0));

        // draw image on red background
        //ferris.draw(&mut display).unwrap();
        display.clear(Rgb565::YELLOW).unwrap();
        info!("Cleared display!");

        Self { display, rd }
    }

    pub fn draw_rectangle(&mut self, color: Rgb565, size: u32) -> Result<()> {
        Rectangle::new(Point::new(0, 0), Size::new(size, size))
            .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
            .draw(&mut self.display)
            .map_err(|_| anyhow!("unable to draw rectangle"))?;

        info!("Draw Rectangle");

        Ok(())
    }

    pub fn set_display_color(&mut self, color: Rgb565, text: &str) -> Result<()> {
        info!("Draw color {text}");

        Rectangle::new(Point::new(50, 50), Size::new(200, 100))
            .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
            .draw(&mut self.display)
            .map_err(|_| anyhow!("unable to draw rectangle"))?;

        // Create a new character style
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

        Text::new(text, Point::new(100, 50), style)
            .draw(&mut self.display)
            .map_err(|_| anyhow!("unable to draw text"))?;

        Ok(())
    }
}
