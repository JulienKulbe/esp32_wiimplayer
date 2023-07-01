use crate::{device::http::HttpClient, model::player_status::PlayerStatus};
use anyhow::Result;
use esp_idf_svc::systime::EspSystemTime;
use log::info;
use std::time::Duration;

use self::player_status::TrackInfo;

pub mod player_status;

static WIIM_URL: &str = "https://192.168.1.48/httpapi.asp?command=getPlayerStatus";

pub struct AudioPlayer<'a> {
    http_client: &'a mut HttpClient,
    status: Option<TrackInfo>,
    last_update: Duration,
}

impl<'a> AudioPlayer<'a> {
    pub fn new(http_client: &'a mut HttpClient) -> AudioPlayer<'a> {
        Self {
            http_client,
            status: None,
            last_update: EspSystemTime {}.now(),
        }
    }

    pub fn update(&mut self) -> Option<&TrackInfo> {
        if !self.should_refresh() {
            return None;
        }

        let data = self.request_server_data().ok();
        self.last_update = EspSystemTime {}.now();

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

    fn should_refresh(&self) -> bool {
        let now = EspSystemTime {}.now();
        let diff = now - self.last_update;
        const UPDATE_TIME: Duration = Duration::from_secs(10);
        diff >= UPDATE_TIME
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
