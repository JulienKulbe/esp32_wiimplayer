use crate::{device::Device, model::player_status::PlayerStatus};
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
        let message = device.http.get_request(url)?;

        let status = serde_json::from_str::<PlayerStatus>(&message)?;

        info!("Artist: {}", status.get_artist()?);
        info!("Title: {}", status.get_title()?);
        info!("Album: {}", status.get_album()?);

        device.tft.draw()?;

        info!("Wait for 1sec...");
        thread::sleep(Duration::from_secs(1));
    }
}
