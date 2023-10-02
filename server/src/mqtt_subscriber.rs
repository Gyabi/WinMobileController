use paho_mqtt::{self as mqtt, SubscribeOptions, AsyncClient, CreateOptionsBuilder, MQTT_VERSION_5};
use std::thread;
use uuid::Uuid;
use std::time::Duration;
use log::{debug, error};

/// MQTTサブスクライバーの起動
/// 
/// * `host` - MQTTブローカーのホスト名
/// * `port` - MQTTブローカーのポート番号
/// * `topics` - サブスクライブするトピックのリスト
/// * `qoss` - サブスクライブするトピックのQoSのリスト
/// * `closures` - メッセージ受信時のコールバック
/// * `use_ssl` - SSLを使用するかどうか
pub fn start_subscribe<F>(
    host: String,
    port: u16,
    topics: Vec<String>,
    qoss: Vec<i32>,
    closures: F,
    use_ssl: bool,
) -> thread::JoinHandle<()>
where
    F: Fn(mqtt::Message) + Send + 'static,
{
    // 設定値を作成
    let create_opts = CreateOptionsBuilder::new()
        .server_uri(format!(
            "{}://{}:{}",
            if use_ssl { "mqtts" } else { "mqtt" },
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
        .finalize();
    
    // メッセージ受信時のコールバック
    client.set_message_callback(move |_cli, msg| {
        debug!("Received message: {:?}", msg);
        closures(msg.unwrap());
    });

    thread::spawn(move || {
        debug!("start subscribe");
        // コネクション作成
        client.connect(conn_opts).wait().unwrap_or_else(|e|{
            error!("Unable to connect: {:?}", e);
            std::process::exit(1);
        });
        
        // サブスクライブ
        let sub_opts = vec![SubscribeOptions::with_retain_as_published(); topics.len()];
        client.subscribe_many_with_options(topics.as_slice(), qoss.as_slice(), &sub_opts, None).wait().unwrap_or_else(|e|{
            error!("Unable to connect: {:?}", e);
            std::process::exit(1);
        });

        // メッセージ受信待ち
        loop {
            thread::sleep(Duration::from_millis(1000));
        }
    })
}