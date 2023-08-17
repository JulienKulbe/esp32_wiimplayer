use log::info;

use super::scroll_textblock::TextBlock;
use crate::{device::tft::TftDisplay, model::player_status::TrackInfo};
use std::sync::mpsc::Receiver;

pub struct PlayerUi {
    display: TftDisplay,
    receiver: Receiver<TrackInfo>,
    text_blocks: [TextBlock; 3],
}

impl PlayerUi {
    pub fn new(receiver: Receiver<TrackInfo>, display: TftDisplay) -> Self {
        let text_blocks = [
            TextBlock::new("Waiting for Wifi", 80),
            TextBlock::new("", 120),
            TextBlock::new("", 160),
        ];

        Self {
            display,
            receiver,
            text_blocks,
        }
    }

    pub fn update(&mut self) {
        if let Ok(info) = self.receiver.try_recv() {
            self.set_track_info(info);
        }

        for text in self.text_blocks.iter_mut() {
            text.update(&mut self.display);
        }
    }

    fn set_track_info(&mut self, data: TrackInfo) {
        info!(
            "Current track: {}-{} ({})",
            data.artist, data.title, data.album
        );

        self.display.set_enable(data.is_playing);
        self.display.reset();

        self.text_blocks[0].set_text(data.artist.as_str());
        self.text_blocks[1].set_text(data.title.as_str());
        self.text_blocks[2].set_text(data.album.as_str());
    }
}
