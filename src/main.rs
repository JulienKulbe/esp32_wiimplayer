use crate::device::Device;
use anyhow::Result;
use log::{error, info};
use std::{thread, time::Duration};

mod device;
mod model;
mod view;

fn main() -> Result<()> {
    let mut device = Device::default();
    thread::sleep(Duration::from_secs(5));

    loop {
        let url = "https://192.168.1.48/httpapi.asp?command=getPlayerStatus";

        info!("Create new request");
        match device.http.get_request(url) {
            Ok(_) => info!("SUCCESS"),
            Err(msg) => error!("ERROR: {msg}"),
        }

        //device.tft.draw()?;

        info!("Wait for 1sec...");
        thread::sleep(Duration::from_secs(1));
    }
}
