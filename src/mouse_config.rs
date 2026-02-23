//! # Mouse Config モジュール
//!
//! Windows の `SystemParametersInfoW` API を使用して、
//! システム全体のマウス設定（カーソル速度・ポインター精度・ボタン入れ替え・スクロール行数）を
//! 取得・設定する。
//!
//! ## 注意事項
//! - 設定はシステムグローバル（全マウスデバイスに共通で適用）
//! - 管理者権限なしで変更可能

/// カーソル速度を取得する（1〜20、デフォルト10）
pub fn get_mouse_speed() -> i32 {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETMOUSESPEED, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };

    let mut speed: i32 = 0;
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            Some(&mut speed as *mut i32 as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
    }
    speed
}

/// カーソル速度を設定する（1〜20）
///
/// SPIF_SENDCHANGE により他アプリにも変更が通知される。
pub fn set_mouse_speed(speed: i32) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_SETMOUSESPEED, SPIF_SENDCHANGE,
    };

    unsafe {
        let _ = SystemParametersInfoW(
            SPI_SETMOUSESPEED,
            0,
            Some(speed as *mut _),
            SPIF_SENDCHANGE,
        );
    }
}

/// 「ポインターの精度を高める」（マウス加速）の状態を取得する
///
/// SPI_GETMOUSE で取得する [i32; 3] のインデックス2 が 0 以外なら有効。
pub fn get_enhance_pointer_precision() -> bool {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETMOUSE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };

    let mut mouse_params: [i32; 3] = [0; 3];
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_GETMOUSE,
            0,
            Some(mouse_params.as_mut_ptr() as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
    }
    mouse_params[2] != 0
}

/// 「ポインターの精度を高める」（マウス加速）を設定する
///
/// 現在の3要素配列を取得し、インデックス2 を書き換えて書き戻す。
pub fn set_enhance_pointer_precision(enabled: bool) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETMOUSE, SPI_SETMOUSE, SPIF_SENDCHANGE,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };

    let mut mouse_params: [i32; 3] = [0; 3];
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_GETMOUSE,
            0,
            Some(mouse_params.as_mut_ptr() as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
    }
    mouse_params[2] = if enabled { 1 } else { 0 };
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_SETMOUSE,
            0,
            Some(mouse_params.as_mut_ptr() as *mut _),
            SPIF_SENDCHANGE,
        );
    }
}

/// マウスボタンの左右入れ替え状態を取得する
pub fn get_swap_buttons() -> bool {
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_SWAPBUTTON};

    unsafe { GetSystemMetrics(SM_SWAPBUTTON) != 0 }
}

/// マウスボタンの左右入れ替えを設定する
///
/// SPI_SETMOUSEBUTTONSWAP は uiParam に値を渡す（pvParam は None）。
pub fn set_swap_buttons(swap: bool) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_SETMOUSEBUTTONSWAP, SPIF_SENDCHANGE,
    };

    unsafe {
        let _ = SystemParametersInfoW(
            SPI_SETMOUSEBUTTONSWAP,
            if swap { 1 } else { 0 },
            None,
            SPIF_SENDCHANGE,
        );
    }
}

/// ホイールのスクロール行数を取得する（通常 1〜20、デフォルト 3）
pub fn get_wheel_scroll_lines() -> i32 {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETWHEELSCROLLLINES, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };

    let mut lines: i32 = 0;
    unsafe {
        let _ = SystemParametersInfoW(
            SPI_GETWHEELSCROLLLINES,
            0,
            Some(&mut lines as *mut i32 as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
    }
    lines
}

/// ホイールのスクロール行数を設定する
///
/// SPI_SETWHEELSCROLLLINES は uiParam に値を渡す（pvParam は None）。
pub fn set_wheel_scroll_lines(lines: i32) {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_SETWHEELSCROLLLINES, SPIF_SENDCHANGE,
    };

    unsafe {
        let _ = SystemParametersInfoW(
            SPI_SETWHEELSCROLLLINES,
            lines as u32,
            None,
            SPIF_SENDCHANGE,
        );
    }
}
