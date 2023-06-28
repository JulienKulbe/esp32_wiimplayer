use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use self::{
    http::HttpClient,
    tft::{TftDisplay, TftPins},
};
use esp_idf_hal::peripherals::Peripherals;

pub mod http;
pub mod tft;

pub struct Device {
    pub http: HttpClient,
    pub tft: TftDisplay,
}

impl Default for Device {
    fn default() -> Self {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_sys::link_patches();
        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();

        let peripherals = Peripherals::take().unwrap();
        let modem = peripherals.modem;
        let tft_pins = TftPins {
            rst: peripherals.pins.gpio5,
            dc: peripherals.pins.gpio7,
            wr: peripherals.pins.gpio8,
            rd: peripherals.pins.gpio9,
            bl: peripherals.pins.gpio38,
            pw: peripherals.pins.gpio15,
            d0: peripherals.pins.gpio39,
            d1: peripherals.pins.gpio40,
            d2: peripherals.pins.gpio41,
            d3: peripherals.pins.gpio42,
            d4: peripherals.pins.gpio45,
            d5: peripherals.pins.gpio46,
            d6: peripherals.pins.gpio47,
            d7: peripherals.pins.gpio48,
        };

        Self {
            http: HttpClient::new(modem),
            tft: TftDisplay::new(tft_pins),
        }
    }
}
