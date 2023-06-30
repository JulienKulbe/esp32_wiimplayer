use crate::{
    device::tft::{TextLine, TftDisplay},
    model::player_status::TrackInfo,
};
use anyhow::Result;

pub struct PlayerUi<'a> {
    display: &'a mut TftDisplay,
}

impl<'a> PlayerUi<'a> {
    pub fn new(display: &'a mut TftDisplay) -> Self {
        Self { display }
    }

    pub fn update(&mut self, data: &TrackInfo) -> Result<()> {
        self.display
            .draw_text(TextLine::Line1, data.artist.as_str())?;
        self.display
            .draw_text(TextLine::Line2, data.title.as_str())?;
        self.display
            .draw_text(TextLine::Line3, data.album.as_str())?;

        Ok(())
    }
}
