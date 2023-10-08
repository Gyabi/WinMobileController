#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn start_mosquitto() -> Result<(), String> {
  println!("Starting mosquitto");
  
  // Ok(())
  Err("Error".to_string())
}

#[tauri::command]
async fn stop_mosquitto() -> Result<(), String> {
  println!("Stopping mosquitto");
  
  Err("Error".to_string())
}

#[tauri::command]
async fn start_server() -> Result<(), String> {
  println!("Starting server");
  
  Ok(())
  // Err("Error".to_string())
}

#[tauri::command]
async fn stop_server() -> Result<(), String> {
  println!("Stopping server");
  
  Ok(())
  // Err("Error".to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_mosquitto, stop_mosquitto, start_server, stop_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

