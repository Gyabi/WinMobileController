use paho_mqtt::{self as mqtt, SubscribeOptions, AsyncClient, CreateOptionsBuilder, MQTT_VERSION_5};
use std::error::Error;
use uuid::Uuid;
use std::time::Duration;
use log::{debug, error};

/// MQTTサブスクライバーの起動
/// 
/// * `host` - MQTTブローカーのホスト名
/// * `port` - MQTTブローカーのポート番号
/// * `topics` - サブスクライブするトピックのリスト
/// * `qoss` - サブスクライブするQoSのリスト
/// * `closures` - メッセージ受信時のコールバック
/// * `ca_path` - CA証明書のパス
/// * `client_cert_path` - クライアント証明書のパス
/// 
/// # Returns
/// 
/// * `Ok(AsyncClient)` - 成功
/// * `Err(Box<dyn Error>)` - 失敗
pub fn start_subscribe<F>(
    host: String,
    port: u16,
    topics: Vec<String>,
    qoss: Vec<i32>,
    closures: F,
    ca_path: String,
    client_cert_path: String,
) -> Result<AsyncClient, Box<dyn Error>>
where
    F: Fn(mqtt::Message) + Send + 'static,
{
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
    let client = AsyncClient::new(create_opts)?;

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

    // メッセージ受信時のコールバック
    client.set_message_callback(move |_cli, msg| {
        debug!("Received message: {:?}", msg);
        if msg.is_none() {
            return;
        }
        closures(msg.unwrap());
    });

    debug!("start subscribe");
    
    // コネクション作成
    let result = client.connect(conn_opts).wait();
    match result {
        Ok(_) => {},
        Err(err) => {
            error!("Unable to connect: {:?}", err);
            return Err(Box::new(err));
        }
    }
        
    // サブスクライブ
    let sub_opts = vec![SubscribeOptions::with_retain_as_published(); topics.len()];
    let result = client.subscribe_many_with_options(topics.as_slice(), qoss.as_slice(), &sub_opts, None).wait();
    match result {
        Ok(_) => {},
        Err(err) => {
            error!("Unable to subscribe: {:?}", err);
            return Err(Box::new(err));
        }
    }

    Ok(client)
}

/// MQTTサブスクライバーを停止する
/// 
/// # Arguments
/// 
/// * `client` - MQTTクライアント
/// * `topics` - サブスクライブするトピックのリスト
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
pub fn stop_subscribe(client: &mut AsyncClient, topics: Vec<String>) -> Result<(), Box<dyn Error>> {
    debug!("stop subscribe");
    // サブスクライブのキャンセル
    let result = client.unsubscribe_many(topics.as_slice()).wait();
    match result {
        Ok(_) => {},
        Err(err) => {
            error!("Unable to unsubscribe: {:?}", err);
            return Err(Box::new(err));
        }
    }    

    // コネクション切断
    let result = client.disconnect(None).wait();
    match result {
        Ok(_) => {
            debug!("MQTT subscriber stopped.");
            Ok(())
        },
        Err(err) => {
            error!("Unable to disconnect: {:?}", err);
            return Err(Box::new(err));
        }
    }
}