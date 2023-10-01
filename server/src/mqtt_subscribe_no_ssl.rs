extern crate paho_mqtt as mqtt;
use mqtt::{Client, Token};
use std::process;
use uuid::Uuid; // uuidクレートを追加

fn main() {
    // MQTTブローカーの接続情報
    let host = "mqtt.example.com";
    let port = 1883; // 通常のMQTTポート
    let topic = "your/topic";
    
    // ランダムなUUIDを生成してクライアントIDとして使用
    let client_id = format!("rust_mqtt_subscriber_{}", Uuid::new_v4());

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
