use log::{error, debug};
use paho_mqtt::{self as mqtt, AsyncClient};
use std::error::Error;
use crate::mqtt_client::mqtt_subscriber::{start_subscribe, stop_subscribe};

/// MQTTパブリッシャーの管理
/// 
/// * `is_running` - MQTTパブリッシャーが起動中かどうか
/// * `client` - MQTTクライアント
pub struct MqttPubManager {
    pub is_running: bool,
    client: mqtt::AsyncClient,
}

impl MqttPubManager {
    /// コンストラクタ
    /// 
    /// # Returns
    /// 
    /// * `MqttPubManager` - MQTTパブリッシャーの管理
    pub fn new() -> Self {
        Self { 
            is_running: false,
            client: mqtt::AsyncClient::new(mqtt::CreateOptionsBuilder::new().finalize()).unwrap(),
        }
    }

    /// MQTTパブリッシャーを開始する
    /// 
    /// * `host` - MQTTブローカーのホスト名
    /// * `port` - MQTTブローカーのポート番号
    /// * `topics` - サブスクライブするトピックのリスト
    /// * `qoss` - サブスクライブするQoSのリスト
    /// * `ca_path` - CA証明書のパス
    /// * `client_cert_path` - クライアント証明書のパス
    /// * `closures` - メッセージ受信時のコールバック
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - 成功
    /// * `Err(Box<dyn Error>)` - 失敗
    pub fn start<F>(
        &mut self,
        host:String,
        port:u16,
        topics: Vec<String>,
        qoss: Vec<i32>,
        ca_path:String,
        client_cert_path:String,
        closures: F,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Fn(mqtt::Message) + Send + 'static,
    {
        if self.is_running {
            return Err("MQTT publisher is already running.".into());
        }

        // MQTT接続を開始し、メンバに生成したクライアントを保持する
        let result = start_subscribe(
            host,
            port,
            topics,
            qoss,
            closures,
            ca_path,
            client_cert_path,
        );
        match result {
            Ok(client) => {
                debug!("MQTT publisher started.");
                self.client = client;
                self.is_running = true;
            },
            Err(err) => {
                error!("Failed to start MQTT subscriber: {}", err);
                return Err(err);
            },
        };

        Ok(())
    }

    /// MQTTパブリッシャーを停止する
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - 成功
    /// * `Err(Box<dyn Error>)` - 失敗
    pub fn stop(&mut self, topics:Vec<String>) -> Result<(), Box<dyn Error>> {
        if !self.is_running {
            return Err("MQTT publisher is not running.".into());
        }

        // MQTT接続を停止する
        let result = stop_subscribe(&mut self.client, topics);
        match result {
            Ok(_) => {
                debug!("MQTT publisher stopped.");
                self.is_running = false;
            },
            Err(err) => {
                error!("Failed to stop MQTT subscriber: {}", err);
                return Err(err);
            },
        };

        Ok(())
    }
}