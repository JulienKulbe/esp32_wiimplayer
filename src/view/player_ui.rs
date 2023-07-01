use super::scroll_textblock::TextBlock;
use crate::{device::tft::TftDisplay, model::player_status::TrackInfo};

pub struct PlayerUi<'a> {
    display: &'a mut TftDisplay,
    text_blocks: [TextBlock; 3],
}

impl<'a> PlayerUi<'a> {
    pub fn new(display: &'a mut TftDisplay) -> Self {
        let text_blocks = [
            TextBlock::new("Waiting for Wifi", 80),
            TextBlock::new("", 120),
            TextBlock::new("", 160),
        ];

        Self {
            display,
            text_blocks,
        }
    }

    pub fn set_track_info(&mut self, data: &TrackInfo) {
        println!(
            "Current track: {}-{} ({})",
            data.artist, data.title, data.album
        );

        self.display.reset();

        self.text_blocks[0].set_text(data.artist.as_str());
        self.text_blocks[1].set_text(data.title.as_str());
        self.text_blocks[2].set_text(data.album.as_str());
    }

    pub fn update(&mut self) {
        for text in self.text_blocks.iter_mut() {
            text.update(self.display);
        }
    }
}
