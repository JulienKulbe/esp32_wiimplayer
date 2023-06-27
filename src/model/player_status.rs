use anyhow::Result;
use serde::Deserialize;

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
    pub fn get_title(&self) -> Result<String> {
        Self::decode_data_to_string(&self.title)
    }

    pub fn get_artist(&self) -> Result<String> {
        Self::decode_data_to_string(&self.artist)
    }

    pub fn get_album(&self) -> Result<String> {
        Self::decode_data_to_string(&self.album)
    }

    fn decode_data_to_string(data: &String) -> Result<String> {
        let decoded = hex::decode(data)?;
        let output = std::str::from_utf8(&decoded)?;
        Ok(output.to_string())
    }
}
