use crate::device::Device;
use anyhow::Result;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use log::info;
use std::{thread, time::Duration};

mod device;
mod model;
mod view;

fn main() -> Result<()> {
    let mut device = Device::default();
    thread::sleep(Duration::from_secs(5));

    loop {
        //let url = "https://192.168.1.48/httpapi.asp?command=getPlayerStatus";
        let url = "http://ifconfig.net/";

        info!("Create new request");
        device.http.get_request(url)?;

        //device.tft.draw()?;

        info!("Wait for 1sec...");
        thread::sleep(Duration::from_secs(1));
    }
}
