use anyhow::Result;
use embedded_svc::{
    http::{client::Client, Method},
    utils::io,
    wifi::{ClientConfiguration, Configuration},
};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, http::client::EspHttpConnection, nvs::EspDefaultNvsPartition,
    wifi::EspWifi,
};
use log::{error, info};

pub struct HttpClient {
    wifi: EspWifi<'static>,
    client: Client<EspHttpConnection>,
}

impl HttpClient {
    pub fn new(modem: Modem) -> Self {
        // WIFI
        let sys_loop = EspSystemEventLoop::take().unwrap();
        let nvs = EspDefaultNvsPartition::take().unwrap();
        let mut wifi_driver = EspWifi::new(modem, sys_loop, Some(nvs)).unwrap();

        wifi_driver
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid: "HubbelNetz".into(),
                password: "kleinerhubbelimgrossennetz".into(),
                ..Default::default()
            }))
            .unwrap();

        wifi_driver.start().unwrap();
        wifi_driver.connect().unwrap();

        let connection = EspHttpConnection::new(&Default::default()).unwrap();
        let client = Client::wrap(connection);

        Self {
            wifi: wifi_driver,
            client,
        }
    }

    pub fn is_connected(&self) -> Result<bool> {
        let connected = self.wifi.is_connected()?;
        Ok(connected)
    }

    pub fn get_request(&mut self) -> Result<()> {
        // Prepare headers and URL
        let headers = [("accept", "text/plain"), ("connection", "close")];
        let url = "http://ifconfig.net/";

        // Send request
        //
        // Note: If you don't want to pass in any headers, you can also use `client.get(url, headers)`.
        let request = self.client.request(Method::Get, url, &headers)?;
        info!("-> GET {}", url);
        let mut response = request.submit()?;

        // Process response
        let status = response.status();
        info!("<- {}", status);
        let (_headers, mut body) = response.split();

        let mut buf = [0u8; 1024];
        let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
        info!("Read {} bytes", bytes_read);
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
    }
}
