extern crate paho_mqtt as mqtt;
use mqtt::{Client, Token, SslOptions};
use std::process;
use uuid::Uuid; // uuidクレートを追加

fn main() {
    // MQTTブローカーの接続情報
    let host = "mqtt.example.com";
    let port = 8883; // SSLポート
    let topic = "your/topic";
    
    // ランダムなUUIDを生成してクライアントIDとして使用
    let client_id = format!("rust_mqtt_subscriber_{}", Uuid::new_v4());

    // SSL証明書とキーのパス
    let ca_cert = "path/to/ca.crt";
    let client_cert = "path/to/client.crt";
    let client_key = "path/to/client.key";

    // MQTTクライアントを初期化
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("ssl://{}:{}", host, port))
        .client_id(&client_id)
        .ssl_options(SslOptions::new(ca_cert, client_cert, client_key).expect("Failed to create SSL options"))
        .finalize();

    let client = Client::new(create_opts).expect("Failed to create MQTT client");

    // MQTTブローカーに接続
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = client.connect(conn_opts).wait() {
        println!("Error: {:?}", e);
        process::exit(1);
    }

    // トピックをサブスクライブ
    let subscribe_opts = mqtt::SubscribeOptionsBuilder::new().qos(1).finalize();
    let mut token = client.subscribe(topic, subscribe_opts).wait().expect("Subscribe failed");
    token.wait().expect("Token wait failed");

    // メッセージを受信して表示
    for _ in 0..10 {
        match client.poll(None) {
            Ok(msg) => {
                if let Some(message) = msg {
                    println!("Received message: {:?}", message);
                }
            }
            Err(e) => {
                println!("Error polling for messages: {:?}", e);
                break;
            }
        }
    }

    // 接続を閉じる
    client.disconnect(None).wait().unwrap();
}
