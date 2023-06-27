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
use log::{error, info};

pub struct HttpClient {
    _wifi: BlockingWifi<EspWifi<'static>>,
}

impl HttpClient {
    pub fn new(modem: Modem) -> Self {
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

        Self { _wifi: wifi_driver }
    }

    pub fn create_client(&mut self) -> Result<Client<EspHttpConnection>> {
        let config = http::client::Configuration {
            ..Default::default()
        };
        let connection = EspHttpConnection::new(&config)?;
        Ok(Client::wrap(connection))
    }

    pub fn get_request(&mut self, url: &str) -> Result<()> {
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
        // let message = std::str::from_utf8(&buf[0..bytes_read])?;
        // let message = message.to_string();

        match std::str::from_utf8(&buf[0..bytes_read]) {
            Ok(body_string) => info!(
                "Response body (truncated to {} bytes): {:?}",
                buf.len(),
                body_string
            ),
            Err(e) => error!("Error decoding response body: {}", e),
        };

        // Drain the remaining response bytes
        while body.read(&mut buf)? > 0 {}

        Ok(())
        //Ok(message)
    }
}
