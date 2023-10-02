use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::mqtt_publisher;
use crate::win;

pub fn pub_sample() {
    let host = "localhost";
    let port = 1883;
    let topic = "test/Topic1";
    let payloads = vec![vec![0x01, 0x02, 0x03], vec![0x04, 0x05, 0x06]];
    let interval = 1000;
    let use_ssl = false;
    let handle = mqtt_publisher::start_publish(
        host.to_string(),
        port,
        topic.to_string(),
        payloads,
        interval,
        use_ssl,
    );
    handle.join().unwrap();
}

/// Windowsの操作サンプル
pub fn win_sample() {
    let sleep_time = std::time::Duration::from_millis(3000);
    std::thread::sleep(sleep_time);
    
    // ALT押下
    let alt_down_input = win::get_key_down_input(VK_LMENU);
    // TAB押下解放
    let tab_down_input = win::get_key_down_input(VK_TAB);
    let tab_up_input = win::get_key_up_input(VK_TAB);
    // ALT解放
    let alt_up_input = win::get_key_up_input(VK_LMENU);
    // マウスカーソルを移動
    let mouse_move_input = win::get_mouse_move_input(2000, 0);
    let mouse_click_down_input = win::get_mouse_click_down_input(win::MouseClickType::LEFT);
    let mouse_click_up_input = win::get_mouse_click_up_input(win::MouseClickType::LEFT);
    // CTRL押下
    let ctrl_down_input = win::get_key_down_input(VK_LCONTROL);
    // ホイール前方回転
    let wheel_input = win::get_mouse_wheel_input(300);
    // CTRL解放
    let ctrl_up_input = win::get_key_up_input(VK_LCONTROL);
    
    
    
    let array = [alt_down_input, tab_down_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let array = [tab_up_input, alt_up_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let array = [mouse_move_input, mouse_click_down_input, mouse_click_up_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let array = [ctrl_down_input, wheel_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let array = [wheel_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let array = [ctrl_up_input];
    win::execute_inputs(&array).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
}