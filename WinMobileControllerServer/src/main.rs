mod win;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

// restapiサーバを立ち上げる
fn main() {
    let sleep_time = std::time::Duration::from_millis(3000);
    std::thread::sleep(sleep_time);

    // // ALT押下
    // let alt_down_input = win::get_key_down_input(VK_LMENU);
    // // TAB押下解放
    // let tab_down_input = win::get_key_down_input(VK_TAB);
    // let tab_up_input = win::get_key_up_input(VK_TAB);
    // // ALT解放
    // let alt_up_input = win::get_key_up_input(VK_LMENU);
    // // マウスカーソルを移動
    // let mouse_move_input = win::get_mouse_move_input(200, 0);
    // // CTRL押下
    // let ctrl_down_input = win::get_key_down_input(VK_LCONTROL);
    // // ホイール前方回転
    // let wheel_input = win::get_mouse_wheel_input(300);
    // // CTRL解放
    // let ctrl_up_input = win::get_key_up_input(VK_LCONTROL);
    

    
    // let array = [alt_down_input, tab_down_input];
    // win::execute_inputs(&array).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));

    // let array = [tab_up_input, alt_up_input];
    // win::execute_inputs(&array).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));

    // let array = [mouse_move_input];
    // win::execute_inputs(&array).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));

    // let array = [ctrl_down_input, wheel_input];
    // win::execute_inputs(&array).unwrap();
    // let array = [wheel_input];
    // win::execute_inputs(&array).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));
    
    // let array = [ctrl_up_input];
    // win::execute_inputs(&array).unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(100));
}
