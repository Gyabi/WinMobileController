use windows::{Win32::Foundation::* , Win32::UI::Input::KeyboardAndMouse::*};

/// 引数として入力した入力操作を実行する
/// 
/// # Arguments
/// 
/// * `inputs` - 入力操作
/// 
/// # Returns
/// 
/// * `Ok(())` - 成功
/// * `Err(WIN32_ERROR)` - 失敗
pub fn execute_inputs(inputs: &[INPUT]) -> Result<(), WIN32_ERROR>{
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);

        let err = GetLastError();
        // print!("err: {}", err.0);
        if err.0 != 0 {
            return Err(err);
        }
    }

    Ok(())
}


/// マウスの移動命令を生成
/// 
/// # Arguments
/// 
/// * `x` - X座標移動量
/// * `y` - Y座標移動量
/// 
/// # Returns
/// 
/// * `INPUT` - マウス移動命令
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

/// マウスホイール命令を生成
/// 
/// # Arguments
/// 
/// * `delta` - ホイール回転量
/// 
/// # Returns
/// 
/// * `INPUT` - マウスホイール命令
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

/// マウスクリック種別
/// 
/// # Variants
/// 
/// * `RIGHT` - 右クリック
/// * `LEFT` - 左クリック
/// * `MIDDLE` - ホイールクリック
#[derive(Copy, Clone)]
pub enum MouseClickType {
    RIGHT,
    LEFT,
    MIDDLE,
}

/// マウスクリック押下入力命令を生成
/// 
/// # Arguments
/// 
/// * `click_type` - マウスクリック種別
/// 
/// # Returns
/// 
/// * `INPUT` - マウスクリック押下入力命令
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

/// マウスクリック解放入力命令を生成
/// 
/// # Arguments
/// 
/// * `click_type` - マウスクリック種別
/// 
/// # Returns
/// 
/// * `INPUT` - マウスクリック解放入力命令
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

/// キーボード入力命令を生成
/// 
/// # Arguments
/// 
/// * `key_code` - キーコード
/// 
/// # Returns
/// 
/// * `INPUT` - キーボード入力命令
pub fn get_key_down_input(key_code: VIRTUAL_KEY) -> INPUT {
    let input = INPUT{
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0{
            ki: KEYBDINPUT{
                wVk: key_code,
                wScan: 0,
                dwFlags: KEYEVENTF_EXTENDEDKEY,
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };

    input
}

/// キーボード解放入力命令を生成
/// 
/// # Arguments
/// 
/// * `key_code` - キーコード
/// 
/// # Returns
/// 
/// * `INPUT` - キーボード解放入力命令
pub fn get_key_up_input(key_code: VIRTUAL_KEY) -> INPUT {
    let input = INPUT{
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0{
            ki: KEYBDINPUT{
                wVk: key_code,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        }
    };

    input
}


// 取得座標系がスクリーン座標で微妙に異なる用なので一旦コメントアウト
// 現在のマウス座標を取得
// pub fn get_current_mouse_position() -> POINT {
    //     let mut point = POINT{x: 0, y: 0}; 
    //     unsafe{
        //         GetCursorPos(&mut point);
        //     }
        
        //     point
// }