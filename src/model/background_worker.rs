use super::{audio_player::AudioPlayer, player_status::TrackInfo};
use crate::device::http::HttpClient;
use anyhow::Result;
use log::info;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

pub fn create_background_thread(http_client: HttpClient) -> Receiver<TrackInfo> {
    let (tx, rx) = std::sync::mpsc::channel::<TrackInfo>();
    let mut worker = BackgroundWorker::new(http_client, tx);

    println!("Create background thread");

    // Execute the runtime in its own thread.
    std::thread::spawn(move || {
        println!("Spawned backround thread");
        let result = worker.run();
        println!("{result:?}");
    });

    rx
}

struct BackgroundWorker {
    tx: Sender<TrackInfo>,
    client: Option<HttpClient>,
}

impl BackgroundWorker {
    fn new(client: HttpClient, tx: Sender<TrackInfo>) -> Self {
        Self {
            tx,
            client: Some(client),
        }
    }

    fn run(&mut self) -> Result<()> {
        info!("Create Audio Player");
        let client = self.client.take().unwrap();
        let mut player = AudioPlayer::new(client);

        loop {
            info!("Update player data");
            if let Some(data) = player.update() {
                self.tx.send(data.clone())?;
            }

            thread::sleep(Duration::from_secs(10));
        }
    }
}
