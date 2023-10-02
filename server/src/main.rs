mod win;
mod mqtt_subscriber;
mod mqtt_publisher;
mod samples;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use paho_mqtt::Message;
use log::{info, debug, error};

fn main() {
    // ログ出力設定
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    // 許容するトピックは以下
    // - WinMobControl/PushMouseButton
    // - WinMobControl/ScrollMouseWheel
    // - WinMobControl/MoveMouseCursor
    // - WinMobControl/Zoom
    let host = "localhost";
    let port = 1883;
    let topics = vec![
        "WinMobControl/PushMouseButton".to_string(),
        "WinMobControl/ScrollMouseWheel".to_string(),
        "WinMobControl/MoveMouseCursor".to_string(),
        "WinMobControl/Zoom".to_string(),
    ];
    let qoss = vec![
        1,1,1,1
    ];

    // mqttサブスクライバ起動    
    let handle_sub = mqtt_subscriber::start_subscribe(
        host.to_string(),
        port,
        topics.clone(),
        qoss.clone(),
        call_back,
        false,
    );
    
    // // 1秒スリープ
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    // // mqttパブリッシャー起動
    // samples::pub_sample();
    
    handle_sub.join().unwrap();

    // samples::win_sample();
}

/// メッセージ受信時のコールバック
/// 
/// * `message` - 受信したメッセージ
fn call_back(message: Message) {
    // メッセージを解析して、実行する操作を決定する
    // ペイロードをjsonデシリアライズ
    // let payload = message.payload_str().unwrap();
    // let json: serde_json::Value = serde_json::from_str(payload).unwrap();

    // トピック名取得
    let topic = message.topic().to_string();

    // topicの値に応じて処理分岐
    match topic.as_str() {
        "WinMobControl/PushMouseButton" => {
            debug!("PushMouseButton");
        },
        "WinMobControl/ScrollMouseWheel" => {
            debug!("ScrollMouseWheel");
        },
        "WinMobControl/MoveMouseCursor" => {
            debug!("MoveMouseCursor");
        },
        "WinMobControl/Zoom" => {
            debug!("Zoom");
        },
        _ => {
            error!("unknown topic");
        }
    }
}