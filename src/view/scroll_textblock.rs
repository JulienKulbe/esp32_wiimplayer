use embedded_graphics::text;
use log::{debug, info};
use serde_json::to_string;

use crate::device::tft::TftDisplay;
use std::cmp;

pub struct TextBlock {
    text: String,
    y: i32,
    needs_update: bool,
    index: CharacterIndex,
}

impl TextBlock {
    pub fn new(text: &str, y: i32) -> Self {
        Self {
            text: text.to_string(),
            y,
            needs_update: true,
            index: CharacterIndex::new(text.chars().count()),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.needs_update = true;
        self.index = CharacterIndex::new(text.len())
    }

    pub fn update(&mut self, display: &mut TftDisplay) {
        // if display need no update we skip the drawing to the screen
        if self.needs_update {
            let text = self.get_current_text_slice();
            display.draw_text(text.as_str(), 0, self.y);
        }

        self.needs_update = self.index.scroll();
    }

    fn get_current_text_slice(&self) -> String {
        let (start, end) = self.index.range();

        info!("Text slice: {:?}", self.index,);
        info!(
            "Text: {}, length: {}, chars: {}",
            self.text,
            self.text.len(),
            self.text.chars().count()
        );

        self.text.chars().skip(start).take(end - start).collect()
    }
}

#[derive(Debug)]
struct CharacterIndex {
    current: i32,
    end: i32,
    length: usize,
    delay: usize,
    direction: i32,
}

impl CharacterIndex {
    // display characters: 20
    // wrap around: 31
    // character size: 16
    // 320 / 16 = 20
    const MAX_CHARACTERS: i32 = 20;
    const STARTUP_DELAY: usize = 20;

    fn new(length: usize) -> Self {
        Self {
            current: 0,
            end: (length as i32) - Self::MAX_CHARACTERS,
            length,
            delay: 0,
            direction: 1,
        }
    }

    fn range(&self) -> (usize, usize) {
        let end = cmp::min(self.length, (self.current + Self::MAX_CHARACTERS) as usize);
        (self.current as usize, end)
    }

    fn scroll(&mut self) -> bool {
        if self.end <= 0 {
            return false;
        }

        // prevent immediate scrolling by adding some delay before
        // we start to move the x coordinate
        if self.delay < Self::STARTUP_DELAY {
            self.delay += 1;
            return false;
        }

        self.current += self.direction;

        if self.current == self.end {
            self.delay = Self::STARTUP_DELAY / 2;
            self.direction = -1;
        }
        if self.current == 0 {
            self.delay = 0;
            self.direction = 1;
        }

        true
    }
}
