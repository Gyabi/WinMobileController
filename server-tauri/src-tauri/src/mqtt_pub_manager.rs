use log::{info, error};
pub struct MqttPubManager {
    pub is_running: bool,
}

impl MqttPubManager {
    pub fn new() -> Self {
        Self { is_running: false }
    }

    pub fn start(&mut self, host:String, port:u16, ca_path:String, client_cert_path:String) {
        if self.is_running {
            return;
        }

        self.is_running = true;

        // 一旦全部出力
        info!("host: {}", host);
        info!("port: {}", port);
        info!("ca_path: {}", ca_path);
        info!("client_cert_path: {}", client_cert_path);
    }

    pub fn stop(&mut self) {
        if !self.is_running {
            return;
        }

        self.is_running = false;
    }
}