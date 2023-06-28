use self::player_status::PlayerStatus;
use crate::device::http::HttpClient;
use anyhow::Result;
use log::info;

pub mod player_status;

static WIIM_URL: &str = "https://192.168.1.48/httpapi.asp?command=getPlayerStatus";

pub struct AudioPlayer<'a> {
    http_client: &'a mut HttpClient,
}

impl<'a> AudioPlayer<'a> {
    pub fn new(http_client: &'a mut HttpClient) -> AudioPlayer<'a> {
        Self { http_client }
    }

    pub fn update(&mut self) -> Result<PlayerStatus> {
        info!("Update WiiM status");
        let message = self.http_client.get_request(WIIM_URL)?;
        let status = serde_json::from_str::<PlayerStatus>(&message)?;

        info!("Artist: {}", status.get_artist()?);
        info!("Title: {}", status.get_title()?);
        info!("Album: {}", status.get_album()?);

        Ok(status)
    }
}
