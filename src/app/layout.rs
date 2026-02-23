//! # Layout サブモジュール
//!
//! UIコントロールの配置・サイズ・テキスト等のデザイン定数を集約する。
//! デザイン変更時はこのファイルのみ修正すればよい。
//!
//! ## デザイン基準
//! - 16px グリッド基準（Windows 11 準拠）
//! - フォント: Yu Gothic UI / Segoe UI
//! - セクション分割でグルーピングを明確化

// === メインウィンドウ ===
pub const MAIN_WINDOW_SIZE: (i32, i32) = (340, 560);
pub const MAIN_WINDOW_POS: (i32, i32) = (300, 100);
pub const MAIN_WINDOW_TITLE: &str = "Multi Mouse Tool";

// === フォント ===
pub const FONT_FAMILY: &str = "Yu Gothic UI";
pub const FONT_SIZE_NORMAL: u32 = 17;      // 9pt 相当
pub const FONT_SIZE_SECTION: u32 = 17;     // セクションタイトル用
pub const FONT_SIZE_DEVICE: u32 = 20;      // 現在のデバイス名 強調用

// === 共通マージン ===
const LEFT: i32 = 16;
const CONTENT_WIDTH: i32 = 300; // 340 - 16*2 - 余裕

// === デバイス一覧 ListView ===
pub const DEVICE_LIST_SIZE: (i32, i32) = (CONTENT_WIDTH, 140);
pub const DEVICE_LIST_POS: (i32, i32) = (LEFT, 12);
pub const DEVICE_LIST_COLUMN_WIDTH: i32 = 280;
pub const DEVICE_LIST_COLUMN_TEXT: &str = "デバイス一覧";

// === 現在のデバイス ===
pub const EDITING_LABEL_TEXT: &str = "（マウスを動かすとデバイスを検知します）";
pub const EDITING_LABEL_SIZE: (i32, i32) = (CONTENT_WIDTH, 24);
pub const EDITING_LABEL_POS: (i32, i32) = (LEFT, 160);

// === デバイス名変更 ===
pub const NAME_LABEL_TEXT: &str = "デバイス名:";
pub const NAME_LABEL_SIZE: (i32, i32) = (80, 20);
pub const NAME_LABEL_POS: (i32, i32) = (LEFT, 188);

pub const NAME_EDIT_SIZE: (i32, i32) = (150, 23);
pub const NAME_EDIT_POS: (i32, i32) = (100, 186);

pub const NAME_BUTTON_TEXT: &str = "変更";
pub const NAME_BUTTON_SIZE: (i32, i32) = (55, 23);
pub const NAME_BUTTON_POS: (i32, i32) = (256, 186);

// === セクション: ポインター設定 ===
pub const SECTION_POINTER_TEXT: &str = "── ポインター設定 ──";
pub const SECTION_POINTER_SIZE: (i32, i32) = (CONTENT_WIDTH, 20);
pub const SECTION_POINTER_POS: (i32, i32) = (LEFT, 220);

pub const SPEED_LABEL_TEXT: &str = "カーソル速度:";
pub const SPEED_LABEL_SIZE: (i32, i32) = (100, 20);
pub const SPEED_LABEL_POS: (i32, i32) = (LEFT, 246);

pub const SPEED_SLIDER_SIZE: (i32, i32) = (230, 30);
pub const SPEED_SLIDER_POS: (i32, i32) = (LEFT, 268);
pub const SPEED_SLIDER_RANGE: std::ops::Range<usize> = 1..20;

pub const SPEED_VALUE_SIZE: (i32, i32) = (36, 20);
pub const SPEED_VALUE_POS: (i32, i32) = (254, 274);

pub const SPEED_UP_TEXT: &str = "▲";
pub const SPEED_UP_SIZE: (i32, i32) = (24, 20);
pub const SPEED_UP_POS: (i32, i32) = (292, 274);

pub const SPEED_DOWN_TEXT: &str = "▼";
pub const SPEED_DOWN_SIZE: (i32, i32) = (24, 20);
pub const SPEED_DOWN_POS: (i32, i32) = (316, 274);

pub const EPP_CHECK_TEXT: &str = "ポインターの精度を高める";
pub const EPP_CHECK_SIZE: (i32, i32) = (200, 20);
pub const EPP_CHECK_POS: (i32, i32) = (LEFT, 302);

// === セクション: ボタン設定 ===
pub const SECTION_BUTTON_TEXT: &str = "── ボタン設定 ──";
pub const SECTION_BUTTON_SIZE: (i32, i32) = (CONTENT_WIDTH, 20);
pub const SECTION_BUTTON_POS: (i32, i32) = (LEFT, 334);

pub const SWAP_CHECK_TEXT: &str = "ボタン左右入れ替え";
pub const SWAP_CHECK_SIZE: (i32, i32) = (160, 20);
pub const SWAP_CHECK_POS: (i32, i32) = (LEFT, 360);

// === セクション: スクロール設定 ===
pub const SECTION_SCROLL_TEXT: &str = "── スクロール設定 ──";
pub const SECTION_SCROLL_SIZE: (i32, i32) = (CONTENT_WIDTH, 20);
pub const SECTION_SCROLL_POS: (i32, i32) = (LEFT, 392);

pub const SCROLL_LABEL_TEXT: &str = "スクロール行数:";
pub const SCROLL_LABEL_SIZE: (i32, i32) = (110, 20);
pub const SCROLL_LABEL_POS: (i32, i32) = (LEFT, 418);

pub const SCROLL_SLIDER_SIZE: (i32, i32) = (230, 30);
pub const SCROLL_SLIDER_POS: (i32, i32) = (LEFT, 440);
pub const SCROLL_SLIDER_RANGE: std::ops::Range<usize> = 1..21;

pub const SCROLL_VALUE_SIZE: (i32, i32) = (36, 20);
pub const SCROLL_VALUE_POS: (i32, i32) = (254, 446);

pub const SCROLL_UP_TEXT: &str = "▲";
pub const SCROLL_UP_SIZE: (i32, i32) = (24, 20);
pub const SCROLL_UP_POS: (i32, i32) = (292, 446);

pub const SCROLL_DOWN_TEXT: &str = "▼";
pub const SCROLL_DOWN_SIZE: (i32, i32) = (24, 20);
pub const SCROLL_DOWN_POS: (i32, i32) = (316, 446);

// === 設定ボタン ===
pub const SETTINGS_BUTTON_TEXT: &str = "詳細設定...";
pub const SETTINGS_BUTTON_SIZE: (i32, i32) = (120, 28);
pub const SETTINGS_BUTTON_POS: (i32, i32) = (110, 490);

// === 設定ウィンドウ ===
pub const SETTINGS_WINDOW_SIZE: (i32, i32) = (300, 340);
pub const SETTINGS_WINDOW_POS: (i32, i32) = (400, 100);
pub const SETTINGS_WINDOW_TITLE: &str = "デバイス設定";

pub const SETTINGS_LIST_SIZE: (i32, i32) = (268, 220);
pub const SETTINGS_LIST_POS: (i32, i32) = (16, 12);
pub const SETTINGS_LIST_COLUMN_WIDTH: i32 = 248;
pub const SETTINGS_LIST_COLUMN_TEXT: &str = "デバイス名";

pub const SETTINGS_MONITOR_TEXT: &str = "監視する";
pub const SETTINGS_MONITOR_SIZE: (i32, i32) = (82, 28);
pub const SETTINGS_MONITOR_POS: (i32, i32) = (16, 242);

pub const SETTINGS_UNMONITOR_TEXT: &str = "監視外にする";
pub const SETTINGS_UNMONITOR_SIZE: (i32, i32) = (82, 28);
pub const SETTINGS_UNMONITOR_POS: (i32, i32) = (106, 242);

pub const SETTINGS_CLOSE_TEXT: &str = "閉じる";
pub const SETTINGS_CLOSE_SIZE: (i32, i32) = (82, 28);
pub const SETTINGS_CLOSE_POS: (i32, i32) = (202, 242);

pub const SETTINGS_RESET_TEXT: &str = "リセット";
pub const SETTINGS_RESET_SIZE: (i32, i32) = (82, 28);
pub const SETTINGS_RESET_POS: (i32, i32) = (109, 280);

// === トレイ ===
pub const TRAY_TIP: &str = "Multi Mouse Tool";
pub const TRAY_ITEM_SHOW_TEXT: &str = "表示";
pub const TRAY_ITEM_EXIT_TEXT: &str = "終了";
