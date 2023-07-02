use super::{audio_player::AudioPlayer, player_status::TrackInfo};
use crate::device::http::HttpClient;
use anyhow::Result;
use log::{error, info};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

pub fn create_background_thread(http_client: HttpClient) -> Receiver<TrackInfo> {
    let (tx, rx) = std::sync::mpsc::channel::<TrackInfo>();
    let mut worker = BackgroundWorker::new(http_client, tx);

    info!("Create background thread");

    // Execute the runtime in its own thread.
    std::thread::spawn(move || {
        let result = worker.run();
        error!("{result:?}");
    });

    rx
}

struct BackgroundWorker {
    tx: Sender<TrackInfo>,
    player: AudioPlayer,
}

impl BackgroundWorker {
    fn new(client: HttpClient, tx: Sender<TrackInfo>) -> Self {
        Self {
            tx,
            player: AudioPlayer::new(client),
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            info!("Update player data");
            if let Some(data) = self.player.update() {
                self.tx.send(data.clone())?;
            }

            thread::sleep(Duration::from_secs(10));
        }
    }
}
