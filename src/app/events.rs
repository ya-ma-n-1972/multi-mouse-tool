//! # Events サブモジュール
//!
//! NWG / Raw Win32 / 設定画面のイベントハンドラ登録を担当する。
//!
//! ## イベントの2系統
//! 1. **NWG イベント**: ウィンドウ操作、トレイ操作、スライダー変更など
//! 2. **Raw Win32 イベント**: WM_INPUT（マウス入力）、WM_DEVICECHANGE（デバイス着脱）

use super::{App, WM_DEVICECHANGE, WM_INPUT};
use crate::device;
use native_windows_gui as nwg;
use std::rc::Rc;

/// メインウィンドウの NWG イベントハンドラを登録する
///
/// 処理対象: OnResize（最小化検知）、OnContextMenu（トレイ）、
/// OnMenuItemSelected、OnHorizontalScroll、OnButtonClick、OnWindowClose
pub fn register_nwg_handler(app: &Rc<App>) -> nwg::EventHandler {
    let evt_ui = Rc::downgrade(app);

    nwg::full_bind_event_handler(
        &app.window.handle,
        move |evt, _evt_data, handle| {
            let Some(ui) = evt_ui.upgrade() else { return };

            match evt {
                nwg::Event::OnResize => {
                    if &handle == &ui.window.handle {
                        let (w, h) = ui.window.size();
                        if w == 0 && h == 0 {
                            ui.on_minimize();
                        }
                    }
                }
                nwg::Event::OnContextMenu => {
                    if &handle == &ui.tray.handle {
                        let (x, y) = nwg::GlobalCursor::position();
                        ui.tray_menu.popup(x, y);
                    }
                }
                nwg::Event::OnMenuItemSelected => {
                    if &handle == &ui.tray_item_show.handle {
                        ui.on_tray_show();
                    } else if &handle == &ui.tray_item_exit.handle {
                        ui.on_tray_exit();
                    }
                }
                nwg::Event::OnHorizontalScroll => {
                    if &handle == &ui.speed_slider.handle {
                        ui.on_speed_change();
                    } else if &handle == &ui.scroll_slider.handle {
                        ui.on_scroll_change();
                    }
                }
                nwg::Event::OnButtonClick => {
                    if &handle == &ui.name_button.handle {
                        ui.on_name_change();
                    } else if &handle == &ui.settings_button.handle {
                        ui.on_open_settings();
                    } else if &handle == &ui.epp_check.handle {
                        ui.on_epp_change();
                    } else if &handle == &ui.swap_check.handle {
                        ui.on_swap_change();
                    } else if &handle == &ui.speed_up_button.handle {
                        ui.on_speed_up();
                    } else if &handle == &ui.speed_down_button.handle {
                        ui.on_speed_down();
                    } else if &handle == &ui.scroll_up_button.handle {
                        ui.on_scroll_up();
                    } else if &handle == &ui.scroll_down_button.handle {
                        ui.on_scroll_down();
                    }
                }
                nwg::Event::OnWindowClose => {
                    if &handle == &ui.window.handle {
                        ui.on_window_close();
                    } else if &handle == &ui.settings_window.handle {
                        ui.on_settings_close();
                    }
                }
                _ => {}
            }
        },
    )
}

/// Raw Win32 イベントハンドラを登録する（WM_INPUT / WM_DEVICECHANGE）
///
/// NWG が標準イベントとして公開しないメッセージを直接受信する。
pub fn register_raw_handler(
    app: &Rc<App>,
) -> Result<nwg::RawEventHandler, nwg::NwgError> {
    let raw_ui = Rc::downgrade(app);

    nwg::bind_raw_event_handler(
        &app.window.handle,
        0x10001,
        move |_hwnd, msg, _w, l| {
            let Some(ui) = raw_ui.upgrade() else { return None };

            match msg {
                WM_INPUT => {
                    if let Some(device_handle) = device::get_raw_input_device(l) {
                        let handles = ui.device_handles.borrow();
                        let found = handles.iter().any(|&h| h == device_handle);
                        drop(handles);
                        if found {
                            ui.highlight_device(device_handle);
                            ui.on_device_switch(device_handle);
                        }
                    }
                }
                WM_DEVICECHANGE => {
                    ui.refresh_device_list();
                }
                _ => {}
            }

            None // Windows のデフォルト処理に委譲
        },
    )
}

/// 設定ウィンドウのイベントハンドラを登録する
///
/// 設定ウィンドウの子コントロール（ボタン）の WM_COMMAND は
/// settings_window に届くため、メインウィンドウとは別に登録が必要。
pub fn register_settings_handler(app: &Rc<App>) -> nwg::EventHandler {
    let settings_ui = Rc::downgrade(app);

    nwg::full_bind_event_handler(
        &app.settings_window.handle,
        move |evt, _evt_data, handle| {
            let Some(ui) = settings_ui.upgrade() else { return };

            if let nwg::Event::OnButtonClick = evt {
                if &handle == &ui.settings_monitor_button.handle {
                    ui.on_settings_monitor();
                } else if &handle == &ui.settings_unmonitor_button.handle {
                    ui.on_settings_unmonitor();
                } else if &handle == &ui.settings_close_button.handle {
                    ui.on_settings_close();
                } else if &handle == &ui.settings_reset_button.handle {
                    ui.on_settings_reset();
                }
            }
        },
    )
}
