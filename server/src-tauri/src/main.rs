#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod mqtt_pub_manager;
mod setting;
mod win_service;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use log::{info, error};

use mqtt_pub_manager::MqttPubManager;
use setting::{Parameter, read_setting, save_setting};
use win_service::{start_service, stop_service};

/// Mosquittoを開始する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn start_mosquitto() -> Result<(), String> {
  info!("Starting mosquitto");
  
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
  info!("Stopping mosquitto");  

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
  info!("Starting server");
  // パラメータを取得
  let parameter = read_setting().unwrap();

  let mut mqtt_pub_manager = MQTT_PUB_MANAGER.lock().unwrap();
  mqtt_pub_manager.start(
      parameter.host,
      parameter.port,
      parameter.ca_path,
      parameter.client_cert_path,
  );
  
  Ok(())
  // Err("Error".to_string())
}

/// MQTTサーバーを停止する
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
async fn stop_server() -> Result<(), String> {
  info!("Stopping server");
  
  let mut mqtt_pub_manager = MQTT_PUB_MANAGER.lock().unwrap();
  mqtt_pub_manager.stop();
  Ok(())
  // Err("Error".to_string())
}

/// 設定パラメータを取得する
/// 
/// # Returns
/// 
/// * `Ok(Parameter)` - 成功
/// * `Err(String)` - 失敗
#[tauri::command]
fn get_setting() -> Result<Parameter, String> {
  info!("Getting setting");

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
  info!("Setting setting");
  
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
  env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_mosquitto, stop_mosquitto, start_server, stop_server, get_setting, set_setting])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

