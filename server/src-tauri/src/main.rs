#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod mqtt_client;
mod setting;
mod win_service;
mod win_input;
mod logic;

use lazy_static::lazy_static;
use serde::de;
use std::sync::{Arc, Mutex};
use log::debug;

use mqtt_client::mqtt_pub_manager::MqttPubManager;
use setting::param::{Parameter, read_setting, save_setting};
use win_service::win_service_control::{start_service, stop_service};
use logic::call_back;

/// Mosquittoを開始する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn start_mosquitto() -> Result<(), String> {
  debug!("Starting mosquitto");
  
  let result = start_service("Mosquitto Broker");
  match result {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string()),
  }
}

/// Mosquittoを停止する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn stop_mosquitto() -> Result<(), String> {
  debug!("Stopping mosquitto");  

  let result = stop_service("Mosquitto Broker");
  match result {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string()),
  }
}

/// MQTTサーバーを開始する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn start_server() -> Result<(), String> {
  debug!("Starting server");
  // パラメータを取得
  let parameter = read_setting().unwrap();
  let topics = vec![
    "WinMobControl/PushMouseButton".to_string(),
    "WinMobControl/ScrollMouseWheel".to_string(),
    "WinMobControl/MoveMouseCursor".to_string(),
    "WinMobControl/Zoom".to_string(),
  ];
  let qoss = vec![
      1,1,1,1
  ];

  let mut mqtt_pub_manager = MQTT_PUB_MANAGER.lock().unwrap();
  let result = mqtt_pub_manager.start(
      parameter.host,
      parameter.port,
      topics,
      qoss,
      parameter.ca_path,
      parameter.client_cert_path,
      call_back,
  );

  match result {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string()),
  }
}

/// MQTTサーバーを停止する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn stop_server() -> Result<(), String> {
  debug!("Stopping server");
  let topics = vec![
    "WinMobControl/PushMouseButton".to_string(),
    "WinMobControl/ScrollMouseWheel".to_string(),
    "WinMobControl/MoveMouseCursor".to_string(),
    "WinMobControl/Zoom".to_string(),
  ];
  
  let mut mqtt_pub_manager = MQTT_PUB_MANAGER.lock().unwrap();
  let result = mqtt_pub_manager.stop(topics);
  match result {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string()),
  }
}

/// 設定パラメータを取得する
/// 
/// # Returns
/// 
/// * `Ok(Parameter)` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
fn get_setting() -> Result<Parameter, String> {
  debug!("Getting setting");

  let parameter = read_setting();
  match parameter {
    Ok(parameter) => Ok(parameter),
    Err(err) => Err(err.to_string()),
  }
}

/// 設定パラメータを保存する
/// 
/// # Arguments
/// 
/// * `parameter` - 設定パラメータ
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command(rename_all = "snake_case")]
fn set_setting(parameter: Parameter) -> Result<(), String> {
  debug!("Setting setting");
  
  let err = save_setting(parameter);
  match err {
    Ok(_) => Ok(()),
    Err(err) => Err(err.to_string()),
  }
}

lazy_static! {
  static ref MQTT_PUB_MANAGER: Arc<Mutex<MqttPubManager>> = Arc::new(Mutex::new(MqttPubManager::new()));
}

fn main() {
  // ログ出力設定
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("off"));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_mosquitto, stop_mosquitto, start_server, stop_server, get_setting, set_setting])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

