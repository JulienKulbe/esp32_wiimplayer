use crate::device::Device;
use anyhow::Result;
use model::background_worker::create_background_thread;
use std::{thread, time::Duration};
use view::player_ui::PlayerUi;

mod device;
mod model;
mod view;

fn main() -> Result<()> {
    let device = Device::default();
    let receiver = create_background_thread(device.http);
    let mut ui = PlayerUi::new(receiver, device.tft);

    loop {
        ui.update();
        thread::sleep(Duration::from_secs_f32(1. / 10.));
    }
}
