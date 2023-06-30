use crate::device::Device;
use anyhow::Result;
use log::info;
use model::AudioPlayer;
use std::{thread, time::Duration};
use view::player_ui::PlayerUi;

mod device;
mod model;
mod view;

fn main() -> Result<()> {
    let mut device = Device::default();
    let mut player = AudioPlayer::new(&mut device.http);
    let mut ui = PlayerUi::new(&mut device.tft);

    loop {
        if let Some(data) = player.update() {
            ui.update(data)?;
        }

        info!("Wait for 1sec...");
        thread::sleep(Duration::from_secs(1));
    }
}
