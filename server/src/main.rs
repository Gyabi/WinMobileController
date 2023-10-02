mod win;
mod mqtt_subscriber;
mod mqtt_publisher;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
fn main() {
    let topics = vec!["test/Topic1".to_string()];
    let qoss = vec![1];
    
    // mqttサブスクライバ起動    
    let handle_sub = mqtt_subscriber::start_subscribe(
        "localhost".to_string(),
        1883,
        topics.clone(),
        qoss.clone(),
        |message| {
            println!("Received message: {:?}", message);
        },
        false,
    );
    handle_sub.join().unwrap();
    // mqttパブリッシャー起動

    // win_sample();
}


fn win_sample() {
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