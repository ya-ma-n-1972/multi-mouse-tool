//! # App モジュール
//!
//! アプリケーションのメインUI（ウィンドウ、トレイ、デバイスリスト、設定パネル）と
//! イベントハンドリングを管理する。
//!
//! ## サブモジュール
//! - [`ui`]: AppUi 構造体、UI 構築（build_ui）、リソース管理
//! - [`events`]: NWG / Raw Win32 / 設定画面のイベントハンドラ登録

pub mod events;
pub mod layout;
pub mod ui;

use native_windows_gui as nwg;
use std::cell::RefCell;

use crate::config;
use crate::device;
use crate::mouse_config;

/// Windows メッセージ定数
pub(crate) const WM_INPUT: u32 = 0x00FF;
pub(crate) const WM_DEVICECHANGE: u32 = 0x0219;

/// アプリケーションの全UIコンポーネントとデバイス情報を保持する構造体
///
/// ## device_handles について
/// `device_list` (ListView) の行インデックスと `device_handles` のインデックスが
/// 1対1で対応する。WM_INPUT で受け取ったハンドルから対応行をハイライトする。
#[derive(Default)]
pub struct App {
    // --- メインウィンドウ ---
    pub(crate) window: nwg::Window,
    pub(crate) icon: nwg::Icon,
    pub(crate) device_list: nwg::ListView,
    pub(crate) device_handles: RefCell<Vec<isize>>,

    // --- フォント ---
    pub(crate) font_normal: nwg::Font,
    pub(crate) font_section: nwg::Font,
    pub(crate) font_device: nwg::Font,

    // --- 設定パネル ---
    pub(crate) editing_label: nwg::Label,
    pub(crate) name_label: nwg::Label,
    pub(crate) name_edit: nwg::TextInput,
    pub(crate) name_button: nwg::Button,

    // --- セクション: ポインター設定 ---
    pub(crate) section_pointer_label: nwg::Label,
    pub(crate) speed_label: nwg::Label,
    pub(crate) speed_value: nwg::Label,
    pub(crate) speed_slider: nwg::TrackBar,
    pub(crate) speed_up_button: nwg::Button,
    pub(crate) speed_down_button: nwg::Button,
    pub(crate) epp_check: nwg::CheckBox,

    // --- セクション: ボタン設定 ---
    pub(crate) section_button_label: nwg::Label,
    pub(crate) swap_check: nwg::CheckBox,

    // --- セクション: スクロール設定 ---
    pub(crate) section_scroll_label: nwg::Label,
    pub(crate) scroll_label: nwg::Label,
    pub(crate) scroll_value: nwg::Label,
    pub(crate) scroll_slider: nwg::TrackBar,
    pub(crate) scroll_up_button: nwg::Button,
    pub(crate) scroll_down_button: nwg::Button,

    pub(crate) settings_button: nwg::Button,

    // --- 設定ウィンドウ ---
    pub(crate) settings_window: nwg::Window,
    pub(crate) settings_device_list: nwg::ListView,
    pub(crate) settings_instance_ids: RefCell<Vec<String>>,
    pub(crate) settings_monitor_button: nwg::Button,
    pub(crate) settings_unmonitor_button: nwg::Button,
    pub(crate) settings_close_button: nwg::Button,
    pub(crate) settings_reset_button: nwg::Button,

    // --- トレイ ---
    pub(crate) tray: nwg::TrayNotification,
    pub(crate) tray_menu: nwg::Menu,
    pub(crate) tray_item_show: nwg::MenuItem,
    pub(crate) tray_item_exit: nwg::MenuItem,

    // --- 内部状態 ---
    pub(crate) config: RefCell<config::AppConfig>,
    pub(crate) device_instance_ids: RefCell<Vec<String>>,
    pub(crate) active_device_index: RefCell<Option<usize>>,
    pub(crate) all_connected_ids: RefCell<Vec<String>>,
    /// プログラムからUI更新中のフラグ（イベントハンドラの意図しない発火を抑制）
    pub(crate) is_updating_ui: RefCell<bool>,
}

impl App {
    // =========================================================================
    // ヘルパーメソッド
    // =========================================================================

    /// アクティブデバイスの config をクロージャで更新して保存する
    ///
    /// on_speed_change / on_epp_change / on_swap_change / on_scroll_change の
    /// 共通パターンを抽出したヘルパー。
    fn update_active_device_config(&self, f: impl FnOnce(&mut config::DeviceConfig)) {
        let selected = *self.active_device_index.borrow();
        let Some(idx) = selected else { return };

        let instance_ids = self.device_instance_ids.borrow();
        let instance_id = instance_ids[idx].clone();
        drop(instance_ids);

        let mut config = self.config.borrow_mut();
        if let Some(dc) = config.find_by_instance_id_mut(&instance_id) {
            f(dc);
        }
        drop(config);
        self.save_config();
    }

    /// 設定保存を実行し、失敗時にトレイ通知を表示する
    fn save_config(&self) {
        let config = self.config.borrow();
        if let Err(msg) = config::save(&config) {
            self.tray.show(
                &format!("設定の保存に失敗しました:\n{}", msg),
                Some("Multi Mouse Tool"),
                Some(nwg::TrayNotificationFlags::ERROR_ICON),
                None,
            );
        }
    }

    /// 設定パネルのコントロールを無効化してリセットする
    fn disable_settings_panel(&self) {
        self.editing_label.set_text(layout::EDITING_LABEL_TEXT);
        self.speed_slider.set_enabled(false);
        self.name_edit.set_enabled(false);
        self.name_button.set_enabled(false);
        self.epp_check.set_enabled(false);
        self.swap_check.set_enabled(false);
        self.scroll_slider.set_enabled(false);
        self.speed_up_button.set_enabled(false);
        self.speed_down_button.set_enabled(false);
        self.scroll_up_button.set_enabled(false);
        self.scroll_down_button.set_enabled(false);
    }

    // =========================================================================
    // デバイスリスト管理
    // =========================================================================

    /// デバイスリストを再構築する
    ///
    /// 起動時の初期表示、WM_DEVICECHANGE 受信時、設定変更後に呼び出される。
    /// 未登録デバイスは現在のOS設定値をデフォルトとして config に自動登録する。
    pub(crate) fn refresh_device_list(&self) {
        let devices = device::enumerate_mice();

        // all_connected_ids を更新（設定ウィンドウのデータソース用）
        let mut all_ids = self.all_connected_ids.borrow_mut();
        all_ids.clear();
        for (_, instance_id, _) in &devices {
            all_ids.push(instance_id.clone());
        }
        drop(all_ids);

        // 未登録デバイスを一括登録
        let mut config = self.config.borrow_mut();
        let current_speed = mouse_config::get_mouse_speed();
        let mut added = false;
        for (vid_pid_name, instance_id, _) in &devices {
            if config.find_by_instance_id(instance_id).is_none() {
                config.devices.push(config::DeviceConfig {
                    instance_id: instance_id.clone(),
                    display_name: vid_pid_name.clone(),
                    speed: current_speed,
                    monitored: true,
                    enhance_pointer_precision: mouse_config::get_enhance_pointer_precision(),
                    swap_buttons: mouse_config::get_swap_buttons(),
                    wheel_scroll_lines: mouse_config::get_wheel_scroll_lines(),
                });
                added = true;
            }
        }
        drop(config);
        if added {
            self.save_config();
        }

        // 監視対象デバイスのみ ListView に表示
        let config = self.config.borrow();
        let mut handles = self.device_handles.borrow_mut();
        let mut instance_ids = self.device_instance_ids.borrow_mut();
        handles.clear();
        instance_ids.clear();

        let count = self.device_list.len();
        for i in (0..count).rev() {
            self.device_list.remove_item(i);
        }

        let mut row_index = 0;
        for (_, instance_id, handle_val) in &devices {
            if let Some(dc) = config.find_by_instance_id(instance_id) {
                if !dc.monitored {
                    continue;
                }
                self.device_list.insert_item(nwg::InsertListViewItem {
                    index: Some(row_index as i32),
                    column_index: 0,
                    text: Some(dc.display_name.clone()),
                    image: None,
                });
                handles.push(*handle_val);
                instance_ids.push(instance_id.clone());
                row_index += 1;
            }
        }
    }

    /// 指定ハンドルのデバイスを ListView 上でハイライトする
    pub(crate) fn highlight_device(&self, device_handle: isize) {
        let handles = self.device_handles.borrow();

        for i in 0..handles.len() {
            self.device_list.select_item(i, false);
        }

        if let Some(idx) = handles.iter().position(|&h| h == device_handle) {
            self.device_list.select_item(idx, true);
        }
    }

    // =========================================================================
    // デバイス切り替え
    // =========================================================================

    /// マウスデバイス操作時: デバイスごとの設定をシステムに適用し、設定パネルに反映
    pub(crate) fn on_device_switch(&self, device_handle: isize) {
        let handles = self.device_handles.borrow();
        let idx = match handles.iter().position(|&h| h == device_handle) {
            Some(i) => i,
            None => return,
        };
        drop(handles);

        // 前回と同じデバイスなら何もしない（UIちらつき防止）
        if *self.active_device_index.borrow() == Some(idx) {
            return;
        }
        *self.active_device_index.borrow_mut() = Some(idx);

        let instance_ids = self.device_instance_ids.borrow();
        let instance_id = instance_ids[idx].clone();
        drop(instance_ids);

        let config = self.config.borrow();
        let Some(dc) = config.find_by_instance_id(&instance_id) else {
            return;
        };
        let speed = dc.speed;
        let epp = dc.enhance_pointer_precision;
        let swap = dc.swap_buttons;
        let scroll = dc.wheel_scroll_lines;
        let display_name = dc.display_name.clone();
        drop(config);

        // OS設定を適用
        mouse_config::set_mouse_speed(speed);
        mouse_config::set_enhance_pointer_precision(epp);
        mouse_config::set_swap_buttons(swap);
        mouse_config::set_wheel_scroll_lines(scroll);

        // 設定パネルに反映（is_updating_ui でイベント発火を抑制）
        *self.is_updating_ui.borrow_mut() = true;

        self.editing_label.set_text(&format!("操作中: {}", display_name));
        self.speed_slider.set_enabled(true);
        self.speed_slider.set_pos(speed as usize);
        self.speed_value.set_text(&speed.to_string());
        self.epp_check.set_check_state(if epp {
            nwg::CheckBoxState::Checked
        } else {
            nwg::CheckBoxState::Unchecked
        });
        self.swap_check.set_check_state(if swap {
            nwg::CheckBoxState::Checked
        } else {
            nwg::CheckBoxState::Unchecked
        });
        self.scroll_slider.set_pos(scroll as usize);
        self.scroll_value.set_text(&scroll.to_string());
        self.name_edit.set_enabled(true);
        self.name_edit.set_text(&display_name);
        self.name_button.set_enabled(true);
        self.epp_check.set_enabled(true);
        self.swap_check.set_enabled(true);
        self.scroll_slider.set_enabled(true);
        self.speed_up_button.set_enabled(true);
        self.speed_down_button.set_enabled(true);
        self.scroll_up_button.set_enabled(true);
        self.scroll_down_button.set_enabled(true);

        *self.is_updating_ui.borrow_mut() = false;
    }

    // =========================================================================
    // 設定変更イベント
    // =========================================================================

    /// カーソル速度スライダー変更時
    pub(crate) fn on_speed_change(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let val = self.speed_slider.pos() as i32;
        mouse_config::set_mouse_speed(val);
        self.speed_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.speed = val);
    }

    /// カーソル速度 ▲ ボタン
    pub(crate) fn on_speed_up(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let current = self.speed_slider.pos() as i32;
        if current >= 20 { return; }
        let val = current + 1;
        self.speed_slider.set_pos(val as usize);
        mouse_config::set_mouse_speed(val);
        self.speed_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.speed = val);
    }

    /// カーソル速度 ▼ ボタン
    pub(crate) fn on_speed_down(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let current = self.speed_slider.pos() as i32;
        if current <= 1 { return; }
        let val = current - 1;
        self.speed_slider.set_pos(val as usize);
        mouse_config::set_mouse_speed(val);
        self.speed_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.speed = val);
    }

    /// 「ポインターの精度を高める」チェックボックス変更時
    pub(crate) fn on_epp_change(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let checked = self.epp_check.check_state() == nwg::CheckBoxState::Checked;
        mouse_config::set_enhance_pointer_precision(checked);
        self.update_active_device_config(|dc| dc.enhance_pointer_precision = checked);
    }

    /// 「ボタン左右入れ替え」チェックボックス変更時
    pub(crate) fn on_swap_change(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let checked = self.swap_check.check_state() == nwg::CheckBoxState::Checked;
        mouse_config::set_swap_buttons(checked);
        self.update_active_device_config(|dc| dc.swap_buttons = checked);
    }

    /// スクロール行数スライダー変更時
    pub(crate) fn on_scroll_change(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let val = self.scroll_slider.pos() as i32;
        mouse_config::set_wheel_scroll_lines(val);
        self.scroll_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.wheel_scroll_lines = val);
    }

    /// スクロール行数 ▲ ボタン
    pub(crate) fn on_scroll_up(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let current = self.scroll_slider.pos() as i32;
        if current >= 20 { return; }
        let val = current + 1;
        self.scroll_slider.set_pos(val as usize);
        mouse_config::set_wheel_scroll_lines(val);
        self.scroll_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.wheel_scroll_lines = val);
    }

    /// スクロール行数 ▼ ボタン
    pub(crate) fn on_scroll_down(&self) {
        if *self.is_updating_ui.borrow() { return; }
        let current = self.scroll_slider.pos() as i32;
        if current <= 1 { return; }
        let val = current - 1;
        self.scroll_slider.set_pos(val as usize);
        mouse_config::set_wheel_scroll_lines(val);
        self.scroll_value.set_text(&val.to_string());
        self.update_active_device_config(|dc| dc.wheel_scroll_lines = val);
    }

    /// 「変更」ボタン: デバイスの表示名を更新
    pub(crate) fn on_name_change(&self) {
        let selected = *self.active_device_index.borrow();
        let Some(idx) = selected else { return };

        let new_name = self.name_edit.text();
        if new_name.is_empty() { return; }

        let instance_ids = self.device_instance_ids.borrow();
        let instance_id = instance_ids[idx].clone();
        drop(instance_ids);

        let mut config = self.config.borrow_mut();
        if let Some(dc) = config.find_by_instance_id_mut(&instance_id) {
            dc.display_name = new_name.clone();
        }
        drop(config);
        self.save_config();

        self.device_list.update_item(idx, nwg::InsertListViewItem {
            index: Some(idx as i32),
            column_index: 0,
            text: Some(new_name.clone()),
            image: None,
        });
        self.editing_label.set_text(&format!("操作中: {}", new_name));
    }

    // =========================================================================
    // ウィンドウ / トレイ
    // =========================================================================

    /// ×ボタン: ウィンドウを非表示にしてトレイ常駐
    pub(crate) fn on_window_close(&self) {
        self.window.set_visible(false);
    }

    /// 最小化: トレイに格納（タスクバーに残さない）
    pub(crate) fn on_minimize(&self) {
        self.window.set_visible(false);
    }

    /// トレイ「表示」: ウィンドウを復元・前面表示
    pub(crate) fn on_tray_show(&self) {
        if let Some(hwnd) = self.window.handle.hwnd() {
            use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_RESTORE};
            let hwnd = windows::Win32::Foundation::HWND(hwnd as *mut _);
            unsafe { let _ = ShowWindow(hwnd, SW_RESTORE); }
        }
        self.window.set_visible(true);
        nwg::Window::set_focus(&self.window);
    }

    /// トレイ「終了」: アプリケーション終了
    pub(crate) fn on_tray_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    // =========================================================================
    // 設定ウィンドウ
    // =========================================================================

    /// 設定ウィンドウを開く
    pub(crate) fn on_open_settings(&self) {
        self.refresh_settings_list();
        self.settings_window.set_visible(true);
        nwg::Window::set_focus(&self.settings_window);
    }

    /// 設定ウィンドウの ListView を再構築する
    fn refresh_settings_list(&self) {
        let all_ids = self.all_connected_ids.borrow();
        let config = self.config.borrow();
        let mut settings_ids = self.settings_instance_ids.borrow_mut();
        settings_ids.clear();

        let count = self.settings_device_list.len();
        for i in (0..count).rev() {
            self.settings_device_list.remove_item(i);
        }

        let mut row = 0;
        for instance_id in all_ids.iter() {
            if let Some(dc) = config.find_by_instance_id(instance_id) {
                let text = if dc.monitored {
                    dc.display_name.clone()
                } else {
                    format!("[監視外] {}", dc.display_name)
                };
                self.settings_device_list.insert_item(nwg::InsertListViewItem {
                    index: Some(row as i32),
                    column_index: 0,
                    text: Some(text),
                    image: None,
                });
                settings_ids.push(instance_id.clone());
                row += 1;
            }
        }
    }

    /// 「監視する」ボタン
    pub(crate) fn on_settings_monitor(&self) {
        self.set_device_monitored(true);
    }

    /// 「監視外にする」ボタン
    pub(crate) fn on_settings_unmonitor(&self) {
        self.set_device_monitored(false);
    }

    /// 監視状態の変更（共通処理）
    fn set_device_monitored(&self, monitored: bool) {
        let selected = self.settings_device_list.selected_item();
        let Some(idx) = selected else { return };

        let settings_ids = self.settings_instance_ids.borrow();
        let instance_id = settings_ids[idx].clone();
        drop(settings_ids);

        let mut config = self.config.borrow_mut();
        if let Some(dc) = config.find_by_instance_id_mut(&instance_id) {
            dc.monitored = monitored;
            let display_name = dc.display_name.clone();
            drop(config);
            self.save_config();

            let text = if monitored {
                display_name
            } else {
                format!("[監視外] {}", display_name)
            };
            self.settings_device_list.update_item(idx, nwg::InsertListViewItem {
                index: Some(idx as i32),
                column_index: 0,
                text: Some(text),
                image: None,
            });
        } else {
            drop(config);
        }

        self.refresh_device_list();
    }

    /// 設定ウィンドウ「閉じる」/ ×ボタン
    pub(crate) fn on_settings_close(&self) {
        self.settings_window.set_visible(false);
        *self.active_device_index.borrow_mut() = None;
        self.disable_settings_panel();
    }

    /// 設定ウィンドウ「リセット」: 全デバイス設定を初期化
    pub(crate) fn on_settings_reset(&self) {
        let mut config = self.config.borrow_mut();
        config.devices.clear();
        drop(config);
        self.save_config();

        self.refresh_device_list();
        self.refresh_settings_list();

        *self.active_device_index.borrow_mut() = None;
        self.disable_settings_panel();
    }
}
