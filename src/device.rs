//! # Device モジュール
//!
//! Windows Raw Input API を使用してマウスデバイスの列挙・入力イベント受信を行う。
//!
//! ## 提供する機能
//! - `enumerate_mice()`: 接続中のマウスデバイスの一覧取得
//! - `register_raw_input()`: Raw Input メッセージ(WM_INPUT)の受信登録
//! - `get_raw_input_device()`: WM_INPUT メッセージからデバイスハンドルを抽出
//!
//! ## デバイスパスの形式
//! `\\?\HID#VID_xxxx&PID_yyyy&...#...#{GUID}`
//! - VID: Vendor ID（メーカー識別子）、PID: Product ID（製品識別子）

use native_windows_gui as nwg;
use std::mem;

/// 接続中のマウスデバイスを列挙し、(VID/PID表示名, デバイスパス, ハンドル値) のリストを返す
///
/// ## 戻り値
/// - 第1要素: VID/PID から生成された表示名（例: "VID_046D PID_C077"）
/// - 第2要素: デバイスパス（インスタンスID）― config でデバイスを識別するキー
/// - 第3要素: Raw Input デバイスハンドル値 ― WM_INPUT でのデバイス照合用
pub fn enumerate_mice() -> Vec<(String, String, isize)> {
    use windows::Win32::UI::Input::{
        GetRawInputDeviceInfoW, GetRawInputDeviceList, RAWINPUTDEVICELIST, RIDI_DEVICENAME,
        RIM_TYPEMOUSE,
    };

    let mut device_count: u32 = 0;
    let size = mem::size_of::<RAWINPUTDEVICELIST>() as u32;

    unsafe {
        GetRawInputDeviceList(None, &mut device_count, size);
    }
    if device_count == 0 {
        return Vec::new();
    }

    let mut devices = vec![RAWINPUTDEVICELIST::default(); device_count as usize];
    let ret =
        unsafe { GetRawInputDeviceList(Some(devices.as_mut_ptr()), &mut device_count, size) };
    if ret == u32::MAX {
        return Vec::new();
    }

    let mut result = Vec::new();

    for device in &devices {
        if device.dwType != RIM_TYPEMOUSE {
            continue;
        }

        let handle_val = device.hDevice.0 as isize;

        // デバイス名（パス）のバッファサイズを取得
        let mut name_size: u32 = 0;
        unsafe {
            GetRawInputDeviceInfoW(device.hDevice, RIDI_DEVICENAME, None, &mut name_size);
        }
        if name_size == 0 {
            result.push(("(名前取得不可)".to_string(), String::new(), handle_val));
            continue;
        }

        // デバイス名（パス）を UTF-16 で取得
        let mut name_buf = vec![0u16; name_size as usize];
        let ret = unsafe {
            GetRawInputDeviceInfoW(
                device.hDevice,
                RIDI_DEVICENAME,
                Some(name_buf.as_mut_ptr() as *mut _),
                &mut name_size,
            )
        };
        if ret == u32::MAX {
            result.push(("(名前取得失敗)".to_string(), String::new(), handle_val));
            continue;
        }

        let full_name = String::from_utf16_lossy(&name_buf)
            .trim_end_matches('\0')
            .to_string();

        let display = extract_vid_pid(&full_name);
        result.push((display, full_name, handle_val));
    }

    result
}

/// デバイスパスから VID/PID 部分を抽出する（例: "VID_046D PID_C077"）
fn extract_vid_pid(path: &str) -> String {
    let upper = path.to_uppercase();

    let vid = upper
        .find("VID_")
        .map(|i| &upper[i..])
        .and_then(|s| s.split(&['#', '&', '\\'][..]).next());

    let pid = upper
        .find("PID_")
        .map(|i| &upper[i..])
        .and_then(|s| s.split(&['#', '&', '\\'][..]).next());

    match (vid, pid) {
        (Some(v), Some(p)) => format!("{} {}", v, p),
        _ => path.to_string(),
    }
}

/// マウスの Raw Input メッセージ受信を登録する
///
/// RIDEV_INPUTSINK フラグにより、ウィンドウが非アクティブでも WM_INPUT を受信可能。
pub fn register_raw_input(window: &nwg::Window) {
    use windows::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE, RIDEV_INPUTSINK};

    let hwnd_val = window.handle.hwnd().expect("HWNDの取得に失敗");

    let device = RAWINPUTDEVICE {
        usUsagePage: 0x01, // Generic Desktop Controls
        usUsage: 0x02,     // Mouse
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: windows::Win32::Foundation::HWND(hwnd_val as *mut _),
    };

    unsafe {
        RegisterRawInputDevices(&[device], mem::size_of::<RAWINPUTDEVICE>() as u32)
            .expect("Raw Input登録に失敗");
    }
}

/// WM_INPUT の lparam からマウスデバイスのハンドル値を抽出する
///
/// マウス以外のデバイスやデータ取得失敗時は None を返す。
pub fn get_raw_input_device(lparam: isize) -> Option<isize> {
    use windows::Win32::UI::Input::{
        GetRawInputData, HRAWINPUT, RAWINPUT, RAWINPUTHEADER, RID_INPUT,
    };

    let mut size: u32 = 0;
    let header_size = mem::size_of::<RAWINPUTHEADER>() as u32;
    let hraw = HRAWINPUT(lparam as *mut _);

    unsafe {
        GetRawInputData(hraw, RID_INPUT, None, &mut size, header_size);
    }
    if size == 0 {
        return None;
    }

    let mut buf = vec![0u8; size as usize];
    let ret = unsafe {
        GetRawInputData(
            hraw,
            RID_INPUT,
            Some(buf.as_mut_ptr() as *mut _),
            &mut size,
            header_size,
        )
    };
    if ret == u32::MAX {
        return None;
    }

    // RAWINPUT 構造体として解釈し、dwType == 0 (RIM_TYPEMOUSE) ならハンドルを返す
    let raw = unsafe { &*(buf.as_ptr() as *const RAWINPUT) };
    if raw.header.dwType == 0 {
        Some(raw.header.hDevice.0 as isize)
    } else {
        None
    }
}
