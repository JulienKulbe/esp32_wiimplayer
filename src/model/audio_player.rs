use super::player_status::TrackInfo;
use crate::{device::http::HttpClient, model::player_status::PlayerStatus};
use anyhow::Result;
use log::info;

static WIIM_URL: &str = "https://192.168.1.48/httpapi.asp?command=getPlayerStatus";

pub struct AudioPlayer {
    http_client: HttpClient,
    status: Option<TrackInfo>,
}

impl AudioPlayer {
    pub fn new(mut http_client: HttpClient) -> AudioPlayer {
        http_client.connect_wifi();

        Self {
            http_client,
            status: None,
        }
    }

    pub fn update(&mut self) -> Option<&TrackInfo> {
        info!("Request server data");
        let data = self.request_server_data().ok();

        // change updated if we have new data received
        let mut updated = None;

        if let Some(current) = data {
            match self.status.as_ref() {
                Some(prev) => {
                    if current != *prev {
                        updated = self.update_track_data(current);
                    }
                }
                None => {
                    updated = self.update_track_data(current);
                }
            };
        }

        updated
    }

    fn request_server_data(&mut self) -> Result<TrackInfo> {
        info!("Update WiiM status");
        let message = self.http_client.get_request(WIIM_URL)?;
        let status = serde_json::from_str::<PlayerStatus>(&message)?;
        let track = status.get_track_info()?;
        Ok(track)
    }

    fn update_track_data(&mut self, data: TrackInfo) -> Option<&TrackInfo> {
        self.status = Some(data);
        self.status.as_ref()
    }
}
