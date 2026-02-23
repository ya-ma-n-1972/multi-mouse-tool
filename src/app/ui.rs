//! # UI サブモジュール
//!
//! AppUi 構造体と NativeUi トレイトの実装（UI 構築）を担当する。
//!
//! ## 設計パターン
//! NWG では UI 構造体(App) とイベントハンドラを分離するため、
//! ラッパー構造体(AppUi) を使用するパターンが推奨されている。

use super::events;
use super::layout;
use super::App;
use crate::config;
use crate::device;
use crate::mouse_config;
use native_windows_gui as nwg;
use nwg::NativeUi;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// App を Rc でラップし、イベントハンドラのライフタイムを管理する
pub struct AppUi {
    inner: Rc<App>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
    raw_handler: RefCell<Option<nwg::RawEventHandler>>,
    settings_handler: RefCell<Option<nwg::EventHandler>>,
}

impl NativeUi<AppUi> for App {
    fn build_ui(mut data: App) -> Result<AppUi, nwg::NwgError> {
        // === フォント ===
        nwg::Font::builder()
            .family(layout::FONT_FAMILY)
            .size(layout::FONT_SIZE_NORMAL)
            .build(&mut data.font_normal)?;

        nwg::Font::builder()
            .family(layout::FONT_FAMILY)
            .size(layout::FONT_SIZE_SECTION)
            .weight(700) // Bold
            .build(&mut data.font_section)?;

        nwg::Font::builder()
            .family(layout::FONT_FAMILY)
            .size(layout::FONT_SIZE_DEVICE)
            .weight(700) // Bold
            .build(&mut data.font_device)?;

        // === アイコン ===
        let embed = nwg::EmbedResource::load(None)?;
        nwg::Icon::builder()
            .source_embed(Some(&embed))
            .source_embed_id(2)
            .size(Some((32, 32)))
            .build(&mut data.icon)?;

        // === メインウィンドウ ===
        nwg::Window::builder()
            .size(layout::MAIN_WINDOW_SIZE)
            .position(layout::MAIN_WINDOW_POS)
            .title(layout::MAIN_WINDOW_TITLE)
            .icon(Some(&data.icon))
            .build(&mut data.window)?;

        // === デバイス一覧 ListView ===
        nwg::ListView::builder()
            .size(layout::DEVICE_LIST_SIZE)
            .position(layout::DEVICE_LIST_POS)
            .parent(&data.window)
            .list_style(nwg::ListViewStyle::Detailed)
            .ex_flags(nwg::ListViewExFlags::FULL_ROW_SELECT | nwg::ListViewExFlags::GRID)
            .build(&mut data.device_list)?;

        data.device_list.insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(layout::DEVICE_LIST_COLUMN_WIDTH),
            text: Some(layout::DEVICE_LIST_COLUMN_TEXT.to_string()),
        });

        // === 現在のデバイス ===
        nwg::Label::builder()
            .text(layout::EDITING_LABEL_TEXT)
            .size(layout::EDITING_LABEL_SIZE)
            .position(layout::EDITING_LABEL_POS)
            .font(Some(&data.font_device))
            .parent(&data.window)
            .build(&mut data.editing_label)?;

        // === デバイス名変更 ===
        nwg::Label::builder()
            .text(layout::NAME_LABEL_TEXT)
            .size(layout::NAME_LABEL_SIZE)
            .position(layout::NAME_LABEL_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.name_label)?;

        nwg::TextInput::builder()
            .size(layout::NAME_EDIT_SIZE)
            .position(layout::NAME_EDIT_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.name_edit)?;

        nwg::Button::builder()
            .text(layout::NAME_BUTTON_TEXT)
            .size(layout::NAME_BUTTON_SIZE)
            .position(layout::NAME_BUTTON_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.name_button)?;

        // === セクション: ポインター設定 ===
        nwg::Label::builder()
            .text(layout::SECTION_POINTER_TEXT)
            .size(layout::SECTION_POINTER_SIZE)
            .position(layout::SECTION_POINTER_POS)
            .font(Some(&data.font_section))
            .parent(&data.window)
            .build(&mut data.section_pointer_label)?;

        nwg::Label::builder()
            .text(layout::SPEED_LABEL_TEXT)
            .size(layout::SPEED_LABEL_SIZE)
            .position(layout::SPEED_LABEL_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.speed_label)?;

        nwg::Label::builder()
            .text("")
            .size(layout::SPEED_VALUE_SIZE)
            .position(layout::SPEED_VALUE_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.speed_value)?;

        nwg::TrackBar::builder()
            .size(layout::SPEED_SLIDER_SIZE)
            .position(layout::SPEED_SLIDER_POS)
            .range(Some(layout::SPEED_SLIDER_RANGE))
            .parent(&data.window)
            .build(&mut data.speed_slider)?;

        nwg::Button::builder()
            .text(layout::SPEED_UP_TEXT)
            .size(layout::SPEED_UP_SIZE)
            .position(layout::SPEED_UP_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.speed_up_button)?;

        nwg::Button::builder()
            .text(layout::SPEED_DOWN_TEXT)
            .size(layout::SPEED_DOWN_SIZE)
            .position(layout::SPEED_DOWN_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.speed_down_button)?;

        nwg::CheckBox::builder()
            .text(layout::EPP_CHECK_TEXT)
            .size(layout::EPP_CHECK_SIZE)
            .position(layout::EPP_CHECK_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.epp_check)?;

        // === セクション: ボタン設定 ===
        nwg::Label::builder()
            .text(layout::SECTION_BUTTON_TEXT)
            .size(layout::SECTION_BUTTON_SIZE)
            .position(layout::SECTION_BUTTON_POS)
            .font(Some(&data.font_section))
            .parent(&data.window)
            .build(&mut data.section_button_label)?;

        nwg::CheckBox::builder()
            .text(layout::SWAP_CHECK_TEXT)
            .size(layout::SWAP_CHECK_SIZE)
            .position(layout::SWAP_CHECK_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.swap_check)?;

        // === セクション: スクロール設定 ===
        nwg::Label::builder()
            .text(layout::SECTION_SCROLL_TEXT)
            .size(layout::SECTION_SCROLL_SIZE)
            .position(layout::SECTION_SCROLL_POS)
            .font(Some(&data.font_section))
            .parent(&data.window)
            .build(&mut data.section_scroll_label)?;

        nwg::Label::builder()
            .text(layout::SCROLL_LABEL_TEXT)
            .size(layout::SCROLL_LABEL_SIZE)
            .position(layout::SCROLL_LABEL_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.scroll_label)?;

        nwg::Label::builder()
            .text("")
            .size(layout::SCROLL_VALUE_SIZE)
            .position(layout::SCROLL_VALUE_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.scroll_value)?;

        nwg::TrackBar::builder()
            .size(layout::SCROLL_SLIDER_SIZE)
            .position(layout::SCROLL_SLIDER_POS)
            .range(Some(layout::SCROLL_SLIDER_RANGE))
            .parent(&data.window)
            .build(&mut data.scroll_slider)?;

        nwg::Button::builder()
            .text(layout::SCROLL_UP_TEXT)
            .size(layout::SCROLL_UP_SIZE)
            .position(layout::SCROLL_UP_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.scroll_up_button)?;

        nwg::Button::builder()
            .text(layout::SCROLL_DOWN_TEXT)
            .size(layout::SCROLL_DOWN_SIZE)
            .position(layout::SCROLL_DOWN_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.scroll_down_button)?;

        nwg::Button::builder()
            .text(layout::SETTINGS_BUTTON_TEXT)
            .size(layout::SETTINGS_BUTTON_SIZE)
            .position(layout::SETTINGS_BUTTON_POS)
            .font(Some(&data.font_normal))
            .parent(&data.window)
            .build(&mut data.settings_button)?;

        // === 設定ウィンドウ ===
        nwg::Window::builder()
            .size(layout::SETTINGS_WINDOW_SIZE)
            .position(layout::SETTINGS_WINDOW_POS)
            .title(layout::SETTINGS_WINDOW_TITLE)
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .build(&mut data.settings_window)?;
        data.settings_window.set_visible(false);

        nwg::ListView::builder()
            .size(layout::SETTINGS_LIST_SIZE)
            .position(layout::SETTINGS_LIST_POS)
            .parent(&data.settings_window)
            .list_style(nwg::ListViewStyle::Detailed)
            .ex_flags(nwg::ListViewExFlags::FULL_ROW_SELECT | nwg::ListViewExFlags::GRID)
            .build(&mut data.settings_device_list)?;

        data.settings_device_list.insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(layout::SETTINGS_LIST_COLUMN_WIDTH),
            text: Some(layout::SETTINGS_LIST_COLUMN_TEXT.to_string()),
        });

        nwg::Button::builder()
            .text(layout::SETTINGS_MONITOR_TEXT)
            .size(layout::SETTINGS_MONITOR_SIZE)
            .position(layout::SETTINGS_MONITOR_POS)
            .font(Some(&data.font_normal))
            .parent(&data.settings_window)
            .build(&mut data.settings_monitor_button)?;

        nwg::Button::builder()
            .text(layout::SETTINGS_UNMONITOR_TEXT)
            .size(layout::SETTINGS_UNMONITOR_SIZE)
            .position(layout::SETTINGS_UNMONITOR_POS)
            .font(Some(&data.font_normal))
            .parent(&data.settings_window)
            .build(&mut data.settings_unmonitor_button)?;

        nwg::Button::builder()
            .text(layout::SETTINGS_CLOSE_TEXT)
            .size(layout::SETTINGS_CLOSE_SIZE)
            .position(layout::SETTINGS_CLOSE_POS)
            .font(Some(&data.font_normal))
            .parent(&data.settings_window)
            .build(&mut data.settings_close_button)?;

        nwg::Button::builder()
            .text(layout::SETTINGS_RESET_TEXT)
            .size(layout::SETTINGS_RESET_SIZE)
            .position(layout::SETTINGS_RESET_POS)
            .font(Some(&data.font_normal))
            .parent(&data.settings_window)
            .build(&mut data.settings_reset_button)?;

        // === トレイアイコン・メニュー ===
        nwg::TrayNotification::builder()
            .parent(&data.window)
            .icon(Some(&data.icon))
            .tip(Some(layout::TRAY_TIP))
            .build(&mut data.tray)?;

        nwg::Menu::builder()
            .popup(true)
            .parent(&data.window)
            .build(&mut data.tray_menu)?;

        nwg::MenuItem::builder()
            .text(layout::TRAY_ITEM_SHOW_TEXT)
            .parent(&data.tray_menu)
            .build(&mut data.tray_item_show)?;

        nwg::MenuItem::builder()
            .text(layout::TRAY_ITEM_EXIT_TEXT)
            .parent(&data.tray_menu)
            .build(&mut data.tray_item_exit)?;

        // === 初期値設定 ===
        let current_speed = mouse_config::get_mouse_speed();
        data.speed_slider.set_pos(current_speed as usize);
        data.speed_value.set_text(&current_speed.to_string());
        data.config = RefCell::new(config::load());

        data.speed_slider.set_enabled(false);
        data.name_edit.set_enabled(false);
        data.name_button.set_enabled(false);
        data.epp_check.set_enabled(false);
        data.swap_check.set_enabled(false);
        data.scroll_slider.set_enabled(false);
        data.speed_up_button.set_enabled(false);
        data.speed_down_button.set_enabled(false);
        data.scroll_up_button.set_enabled(false);
        data.scroll_down_button.set_enabled(false);

        // === Raw Input 登録 ===
        device::register_raw_input(&data.window);

        // === AppUi ラップ & 初期化 ===
        let ui = AppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
            raw_handler: Default::default(),
            settings_handler: Default::default(),
        };

        ui.inner.refresh_device_list();

        // === イベントハンドラ登録 ===
        *ui.default_handler.borrow_mut() = Some(events::register_nwg_handler(&ui.inner));
        *ui.raw_handler.borrow_mut() = Some(events::register_raw_handler(&ui.inner)?);
        *ui.settings_handler.borrow_mut() = Some(events::register_settings_handler(&ui.inner));

        Ok(ui)
    }
}

impl Drop for AppUi {
    fn drop(&mut self) {
        if let Some(h) = self.default_handler.borrow().as_ref() {
            nwg::unbind_event_handler(h);
        }
        if let Some(h) = self.raw_handler.borrow().as_ref() {
            nwg::unbind_raw_event_handler(h);
        }
        if let Some(h) = self.settings_handler.borrow().as_ref() {
            nwg::unbind_event_handler(h);
        }
    }
}

impl Deref for AppUi {
    type Target = App;
    fn deref(&self) -> &App {
        &self.inner
    }
}
