use windows::Win32::UI::Input::KeyboardAndMouse::*;
use paho_mqtt::Message;
use log::{info, error};
use serde::{Deserialize, Serialize};

/// マウスボタンを定義したenum
/// 
/// * `Left` - 左ボタン
/// * `Right` - 右ボタン
/// * `Wheel` - ホイールボタン
#[derive(Serialize, Deserialize, Debug)]
pub enum MouseButton {
    Left=0,
    Right=1,
    Wheel=2,
}

/// マウスボタン押下時のメッセージペイロード
/// 
/// * `button` - 押下したボタン
#[derive(Serialize, Deserialize)]
pub struct PushMouseButtonPayload {
    pub button: MouseButton,
}

/// マウスホイールスクロール時のメッセージペイロード
/// 
/// * `delta` - スクロール量
#[derive(Serialize, Deserialize)]
pub struct ScrollMouseWheelPayload {
    pub delta: i32,
}

/// マウスカーソル移動時のメッセージペイロード
/// 
/// * `delta_x` - X軸方向の移動量
/// * `delta_y` - Y軸方向の移動量
#[derive(Serialize, Deserialize)]
pub struct MoveMouseCursorPayload {
    pub delta_x: i32,
    pub delta_y: i32,
}

/// マウスホイールズーム時のメッセージペイロード
/// 
/// * `delta` - ズーム量
#[derive(Serialize, Deserialize)]
pub struct ZoomPayload {
    pub delta: i32,
}

/// メッセージ受信時のコールバック
/// 
/// * `message` - 受信したメッセージ
pub fn call_back(message: Message) {
    // トピック名取得
    let topic = message.topic().to_string();

    // topicの値に応じて処理分岐
    match topic.as_str() {
        "WinMobControl/PushMouseButton" => {
            // メッセージのデシリアライズ
            let payload: PushMouseButtonPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onPush: {:?}", payload.button);
        },
        "WinMobControl/ScrollMouseWheel" => {
            // メッセージのデシリアライズ
            let payload: ScrollMouseWheelPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onScroll: {:?}", payload.delta);
        },
        "WinMobControl/MoveMouseCursor" => {
            // メッセージのデシリアライズ
            let payload: MoveMouseCursorPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onMove: {:?}, {:?}", payload.delta_x, payload.delta_y);
        },
        "WinMobControl/Zoom" => {
            // メッセージのデシリアライズ
            let payload: ZoomPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onZoom: {:?}", payload.delta);
        },
        _ => {
            error!("unknown topic");
        }
    }
}