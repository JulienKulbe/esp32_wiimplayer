use crate::device::Device;
use anyhow::Result;
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
            ui.set_track_info(data);
        }

        ui.update();

        thread::sleep(Duration::from_secs_f32(1. / 10.));
    }
}
