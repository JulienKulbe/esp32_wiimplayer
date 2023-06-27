use anyhow::{bail, Result};
use embedded_svc::{
    http::{client::Client, Method},
    utils::io,
    wifi::{self, AuthMethod, ClientConfiguration},
};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::{self, client::EspHttpConnection},
    nvs::EspDefaultNvsPartition,
    tls::X509,
    wifi::{BlockingWifi, EspWifi},
};
use log::{error, info};

pub struct HttpClient {
    _wifi: BlockingWifi<EspWifi<'static>>,
    client: Client<EspHttpConnection>,
}

impl HttpClient {
    pub fn new(modem: Modem) -> Self {
        // WIFI
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

        let config = http::client::Configuration {
            ..Default::default()
        };
        let connection = EspHttpConnection::new(&config).unwrap();
        let client = Client::wrap(connection);

        info!("Created and connected Wifi");

        Self {
            _wifi: wifi_driver,
            client,
        }
    }

    pub fn get_request(&mut self, url: &str) -> Result<()> {
        // if !self.is_connected()? {
        //     bail!("WiFi not connected");
        // }

        // Prepare headers and URL
        let headers = [("accept", "text/plain"), ("connection", "close")];

        // Send request
        //
        // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
        let request = self.client.get(url)?;
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
