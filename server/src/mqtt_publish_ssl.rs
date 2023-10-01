extern crate paho_mqtt as mqtt;
use mqtt::{Client, Message, SslOptions, Token};
use std::process;
use uuid::Uuid; // uuidクレートを追加

fn main() {
    // MQTTブローカーの接続情報
    let host = "mqtt.example.com";
    let port = 8883; // SSLポート
    let topic = "your/topic";
    let payload = "Hello, MQTT from Rust with SSL!";
    
    // ランダムなUUIDを生成してクライアントIDとして使用
    let client_id = format!("rust_mqtt_publisher_{}", Uuid::new_v4());

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

    // メッセージを送信
    let message = Message::new(topic, payload, 1);
    if let Err(e) = client.publish(message).wait() {
        println!("Error publishing message: {:?}", e);
        process::exit(1);
    }

    // 接続を閉じる
    client.disconnect(None).wait().unwrap();
}
