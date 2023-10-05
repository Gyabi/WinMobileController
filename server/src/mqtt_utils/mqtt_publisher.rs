use paho_mqtt::{self as mqtt, AsyncClient, CreateOptionsBuilder, MQTT_VERSION_5};
use std::thread;
use uuid::Uuid;
use std::time::Duration;
use log::{debug, error};

/// MQTTパブリッシャーの起動
/// 
/// * `host` - MQTTブローカーのホスト名
/// * `port` - MQTTブローカーのポート番号
/// * `topic` - パブリッシュするトピック
/// * `payloads` - パブリッシュするメッセージのリスト
/// * `interval` - メッセージ送信間隔
/// * `ca_path` - CA証明書のパス
/// * `client_cert_path` - クライアント証明書のパス
pub fn start_publish(
    host: String,
    port: u16,
    topic: String,
    payloads: Vec<Vec<u8>>,
    interval: u64,
    ca_path: String,
    client_cert_path: String,
) -> thread::JoinHandle<()> {
    // 設定値を作成
    let create_opts = CreateOptionsBuilder::new()
        .server_uri(format!(
            "mqtts://{}:{}",
            host,
            port
        ))
        .client_id(Uuid::new_v4().to_string())
        .finalize();

    // クライアントを作成
    let client = AsyncClient::new(create_opts).unwrap_or_else(|e|{
        error!("Error creating the client: {:?}", e);
        std::process::exit(1);
    });

    // コネクション作成準備
    let conn_opts = mqtt::ConnectOptionsBuilder::with_mqtt_version(MQTT_VERSION_5)
        .keep_alive_interval(Duration::from_secs(20)) // 20秒間隔でPINGREQを送信
        .clean_start(true) //一度接続が切れたときに、サーバー側に残っているメッセージを削除する
        .ssl_options(
            // SSL設定
            mqtt::SslOptionsBuilder::new()
                .trust_store(ca_path).unwrap()
                .key_store(client_cert_path).unwrap()
                .finalize(),
        )
        .finalize();

    thread::spawn(move || {
        debug!("start subscribe");
        // コネクション作成
        client.connect(conn_opts).wait().unwrap_or_else(|e|{
            error!("Unable to connect: {:?}", e);
            std::process::exit(1);
        });
        
        // メッセージ送信
        for payload in payloads {
            let message = mqtt::Message::new(topic.clone(), payload, 1);
            client.publish(message).wait().unwrap_or_else(|e|{
                error!("Unable to connect: {:?}", e);
                std::process::exit(1);
            });
            thread::sleep(Duration::from_millis(interval));
        }

        println!("finish publish");

        // コネクション切断
        client.disconnect(None).wait().unwrap_or_else(|e|{
            println!("Error disconnecting: {:?}", e);
            std::process::exit(1);
        });
    })
}