mod win;
mod mqtt_utils;
mod samples;
mod logic;

use mqtt_utils::mqtt_subscriber;
use logic::call_back;

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
    
    handle_sub.join().unwrap();

    // // 1秒スリープ
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    // // mqttパブリッシャー起動
    // samples::pub_sample();
    

    // samples::win_sample();
}
