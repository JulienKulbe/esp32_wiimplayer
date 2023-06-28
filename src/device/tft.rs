use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use anyhow::Result;
use display_interface_parallel_gpio::*;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use esp_idf_hal::{delay::Ets, gpio::*};
use log::info;
use mipidsi::{models::ST7789, Builder, Display, Orientation};
use profont::*;

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

pub enum TextLine {
    Line1,
    Line2,
    Line3,
}

pub struct TftDisplay {
    display: LgDisplay,
    _rd: PinDriver<'static, Gpio9, Output>,
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
            .with_orientation(Orientation::Landscape(true))
            .with_invert_colors(mipidsi::ColorInversion::Inverted)
            .init(&mut delay, Some(rst))
            .unwrap();

        // turn on the backlight
        backlight.set_high().unwrap();

        display.clear(Rgb565::BLACK).unwrap();
        info!("Cleared display!");

        Self { display, _rd: rd }
    }

    pub fn draw_text(&mut self, line: TextLine, text: &str) -> Result<()> {
        let y_pos = 80 + line as i32 * 40;

        // draw rectangle
        Rectangle::new(Point::new(0, y_pos - 24), Size::new(320, 30))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut self.display)
            .unwrap();

        // draw text
        let text_style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::WHITE);
        Text::new(text, Point::new(10, y_pos), text_style)
            .draw(&mut self.display)
            .unwrap();
        Ok(())
    }
}
