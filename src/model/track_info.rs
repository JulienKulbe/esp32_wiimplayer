use anyhow::Result;

#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub time: String,
}

impl TrackInfo {
    pub fn new() -> Result<TrackInfo> {
        Ok(Self {
            artist: "Artist".to_string(),
            album: "Album".to_string(),
            title: "Title".to_string(),
            time: "00:00".to_string(),
        })
    }
}
