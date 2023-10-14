use windows::Win32::UI::Input::KeyboardAndMouse::*;
use paho_mqtt::Message;
use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::win_input::win_input_control::{self, MouseClickType};

/// マウスボタンを定義したenum
/// 
/// # Variants
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
/// # Arguments
/// 
/// * `button` - 押下したボタン
#[derive(Serialize, Deserialize)]
pub struct PushMouseButtonPayload {
    pub button: MouseButton,
}

/// マウスホイールスクロール時のメッセージペイロード
/// 
/// # Arguments
/// 
/// * `delta` - スクロール量
#[derive(Serialize, Deserialize)]
pub struct ScrollMouseWheelPayload {
    pub delta: i32,
}

/// マウスカーソル移動時のメッセージペイロード
/// 
/// # Arguments
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
/// # Arguments
/// 
/// * `delta` - ズーム量
#[derive(Serialize, Deserialize)]
pub struct ZoomPayload {
    pub delta: i32,
}

/// メッセージを受信した際のコールバック関数
/// 
/// # Arguments
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
            let button = match payload.button {
                MouseButton::Left => MouseClickType::LEFT,
                MouseButton::Right => MouseClickType::RIGHT,
                MouseButton::Wheel => MouseClickType::MIDDLE,
            };
            
            // マウスボタン押下
            let mouse_click_down_input = win_input_control::get_mouse_click_down_input(button);
            // マウスボタン解放
            let mouse_click_up_input = win_input_control::get_mouse_click_up_input(button);
            let array = [mouse_click_down_input, mouse_click_up_input];
            win_input_control::execute_inputs(&array).unwrap();
        },
        "WinMobControl/ScrollMouseWheel" => {
            // メッセージのデシリアライズ
            let payload: ScrollMouseWheelPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onScroll: {:?}", payload.delta);
            
            let wheel_input = win_input_control::get_mouse_wheel_input(payload.delta);
            let array = [wheel_input];
            win_input_control::execute_inputs(&array).unwrap();
        },
        "WinMobControl/MoveMouseCursor" => {
            // メッセージのデシリアライズ
            let payload: MoveMouseCursorPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onMove: {:?}, {:?}", payload.delta_x, payload.delta_y);

            let mouse_move_input = win_input_control::get_mouse_move_input(payload.delta_x, payload.delta_y);
            let array = [mouse_move_input];
            win_input_control::execute_inputs(&array).unwrap();
        },
        "WinMobControl/Zoom" => {
            // メッセージのデシリアライズ
            let payload: ZoomPayload = serde_json::from_slice(message.payload()).unwrap();
            info!("onZoom: {:?}", payload.delta);
            // CTRL押下
            let ctrl_down_input = win_input_control::get_key_down_input(VK_LCONTROL);
            // ホイール前方回転
            let wheel_input = win_input_control::get_mouse_wheel_input(payload.delta);
            // CTRL解放
            let ctrl_up_input = win_input_control::get_key_up_input(VK_LCONTROL);

            let array = [ctrl_down_input, wheel_input];
            win_input_control::execute_inputs(&array).unwrap();
            let array = [ctrl_up_input];
            win_input_control::execute_inputs(&array).unwrap();
        },
        _ => {
            error!("unknown topic");
        }
    }
}