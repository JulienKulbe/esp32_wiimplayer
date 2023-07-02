use anyhow::Result;
use embedded_svc::{
    http::client::Client,
    utils::io,
    wifi::{self, AuthMethod, ClientConfiguration},
};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::{self, client::EspHttpConnection},
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use log::info;

pub struct HttpClient {
    modem: Option<Modem>,
}

impl HttpClient {
    pub fn new(modem: Modem) -> Self {
        Self { modem: Some(modem) }
    }

    pub fn connect_wifi(&mut self) {
        let modem = self.modem.take().unwrap();
        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();
        let mut wifi_driver = BlockingWifi::wrap(
            EspWifi::new(modem, sys_loop.clone(), Some(nvs)).unwrap(),
            sys_loop,
        )
        .unwrap();

        wifi_driver
            .set_configuration(&wifi::Configuration::Client(ClientConfiguration {
                ssid: "HubbelNetz".into(),
                password: "kleinerhubbelimgrossennetz".into(),
                auth_method: AuthMethod::WPA2Personal,
                ..Default::default()
            }))
            .unwrap();

        wifi_driver.start().unwrap();
        info!("Wifi started");

        wifi_driver.connect().unwrap();
        info!("Wifi connect");

        wifi_driver.wait_netif_up().unwrap();
        info!("Wifi connected");

        // Keep wifi running beyond when this function returns (forever)
        // Do not call this if you ever want to stop or access it later.
        // Otherwise it should be returned from this function and kept somewhere
        // so it does not go out of scope.
        // https://doc.rust-lang.org/stable/core/mem/fn.forget.html
        core::mem::forget(wifi_driver);
    }

    pub fn create_client(&mut self) -> Result<Client<EspHttpConnection>> {
        let config = http::client::Configuration {
            ..Default::default()
        };
        let connection = EspHttpConnection::new(&config)?;
        Ok(Client::wrap(connection))
    }

    pub fn get_request(&mut self, url: &str) -> Result<String> {
        // create HTTP client
        let mut client = self.create_client()?;

        // Send request
        let request = client.get(url)?;
        info!("-> GET {}", url);
        let mut response = request.submit()?;

        // Process response
        let status = response.status();
        info!("<- {}", status);
        let (_headers, mut body) = response.split();

        let mut buf = [0u8; 2048];
        let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
        info!("Read {} bytes", bytes_read);
        let message = std::str::from_utf8(&buf[0..bytes_read])?;
        let message = message.to_string();

        Ok(message)
    }
}
