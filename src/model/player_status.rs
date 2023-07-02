use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct TrackInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerStatus {
    #[serde(alias = "Title")]
    title: String,
    #[serde(alias = "Artist")]
    artist: String,
    #[serde(alias = "Album")]
    album: String,
}

impl PlayerStatus {
    pub fn get_track_info(&self) -> Result<TrackInfo> {
        Ok(TrackInfo {
            title: self.get_title()?,
            artist: self.get_artist()?,
            album: self.get_album()?,
        })
    }

    fn get_title(&self) -> Result<String> {
        Self::decode_data_to_string(&self.title)
    }

    fn get_artist(&self) -> Result<String> {
        Self::decode_data_to_string(&self.artist)
    }

    fn get_album(&self) -> Result<String> {
        Self::decode_data_to_string(&self.album)
    }

    fn decode_data_to_string(data: &String) -> Result<String> {
        let decoded = hex::decode(data)?;
        let output = std::str::from_utf8(&decoded)?;
        Ok(output.to_string())
    }
}
