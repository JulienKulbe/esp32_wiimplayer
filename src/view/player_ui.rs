use crate::{
    device::tft::{TextLine, TftDisplay},
    model::player_status::PlayerStatus,
};
use anyhow::Result;

pub struct PlayerUi<'a> {
    display: &'a mut TftDisplay,
}

impl<'a> PlayerUi<'a> {
    pub fn new(display: &'a mut TftDisplay) -> Self {
        Self { display }
    }

    pub fn update(&mut self, data: PlayerStatus) -> Result<()> {
        self.display
            .draw_text(TextLine::Line1, data.get_artist()?.as_str())?;
        self.display
            .draw_text(TextLine::Line2, data.get_title()?.as_str())?;
        self.display
            .draw_text(TextLine::Line3, data.get_album()?.as_str())?;

        Ok(())
    }
}
