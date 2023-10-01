use paho_mqtt as mqtt;
use mqtt::{Client, Message, Token};
use std::process;
use uuid::Uuid; // uuidクレートを追加

fn main() {
    // MQTTブローカーの接続情報
    let host = "mqtt.example.com";
    let port = 1883; // 通常のMQTTポート
    let topic = "your/topic";
    let payload = "Hello, MQTT from Rust without SSL!";
    
    // ランダムなUUIDを生成してクライアントIDとして使用
    let client_id = format!("rust_mqtt_publisher_{}", Uuid::new_v4());

    // MQTTクライアントを初期化
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("tcp://{}:{}", host, port))
        .client_id(&client_id)
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
