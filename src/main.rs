use crate::device::Device;
use anyhow::Result;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use std::{thread, time::Duration};

mod device;

fn main() -> Result<()> {
    let mut device = Device::default();

    loop {
        // if device.http.is_connected()? {
        //     info!("Wifi conected!");
        //     device.http.get_request()?;
        // } else {
        //     info!("Wifi failed!");
        // }

        //device.tft.set_display_color(Rgb565::GREEN, "GREEN")?;
        device.tft.draw_rectangle(Rgb565::RED, 300)?;
        device.tft.draw_rectangle(Rgb565::YELLOW, 200)?;
        device.tft.draw_rectangle(Rgb565::GREEN, 100)?;
        device.tft.draw_rectangle(Rgb565::BLUE, 50)?;

        thread::sleep(Duration::from_secs(5));
    }
}
