use windows::{Win32::{UI::WindowsAndMessaging::*, Foundation::* }, Win32::UI::Input::KeyboardAndMouse::*};

// 入力実行
pub fn execute_inputs(inputs: &[INPUT]) -> Result<(), WIN32_ERROR>{
    let result = unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32)
    };

    if result == 0 {
        return Err(unsafe{GetLastError()});
    }

    Ok(())
}


// マウス移動入力
pub fn get_mouse_move_input(x: i32, y: i32) -> INPUT {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0{
            mi: MOUSEINPUT{
                dx: x,
                dy: y,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };
    
    input
}

// マウスホイール入力
pub fn get_mouse_wheel_input(delta: i32) -> INPUT {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0{
            mi: MOUSEINPUT{
                dx: 0,
                dy: 0,
                mouseData: delta,
                dwFlags: MOUSEEVENTF_WHEEL,
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };

    input
}


pub enum MouseClickType {
    RIGHT,
    LEFT,
    MIDDLE,
}

// マウスクリック押下入力
pub fn get_mouse_click_down_input(click_type: MouseClickType) -> INPUT {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0{
            mi: MOUSEINPUT{
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: match click_type {
                    MouseClickType::RIGHT => MOUSEEVENTF_RIGHTDOWN,
                    MouseClickType::LEFT => MOUSEEVENTF_LEFTDOWN,
                    MouseClickType::MIDDLE => MOUSEEVENTF_MIDDLEDOWN,
                },
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };

    input
}

// マウスクリック解放入力
pub fn get_mouse_click_up_input(click_type: MouseClickType) -> INPUT {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0{
            mi: MOUSEINPUT{
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: match click_type {
                    MouseClickType::RIGHT => MOUSEEVENTF_RIGHTUP,
                    MouseClickType::LEFT => MOUSEEVENTF_LEFTUP,
                    MouseClickType::MIDDLE => MOUSEEVENTF_MIDDLEUP,
                },
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };
    
    input
}

// キーボード関連は動作が安定しないので一旦コメントアウト
// // キーボード押下入力
// pub fn get_key_down_input(key_code: VIRTUAL_KEY) -> INPUT {
//     let input = INPUT{
//         r#type: INPUT_KEYBOARD,
//         Anonymous: INPUT_0{
//             ki: KEYBDINPUT{
//                 wVk: key_code,
//                 wScan: 0,
//                 dwFlags: KEYEVENTF_EXTENDEDKEY,
//                 time: 0,
//                 dwExtraInfo: 0,
//             },
//         }
//     };

//     input
// }
// // キーボード解放入力
// pub fn get_key_up_input(key_code: VIRTUAL_KEY) -> INPUT {
//     let input = INPUT{
//         r#type: INPUT_KEYBOARD,
//         Anonymous: INPUT_0{
//             ki: KEYBDINPUT{
//                 wVk: key_code,
//                 wScan: 0,
//                 dwFlags: KEYEVENTF_KEYUP,
//                 time: 0,
//                 dwExtraInfo: 0,
//             },
//         }
//     };

//     input
// }


// 取得座標系がスクリーン座標で微妙に異なる用なので一旦コメントアウト
// 現在のマウス座標を取得
// pub fn get_current_mouse_position() -> POINT {
    //     let mut point = POINT{x: 0, y: 0}; 
    //     unsafe{
        //         GetCursorPos(&mut point);
        //     }
        
        //     point
// }