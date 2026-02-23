//! # Config モジュール
//!
//! デバイスごとの設定（カーソル速度・表示名等）を JSON ファイルとして永続化する。
//!
//! ## 保存先
//! `%APPDATA%\multi-mouse-tool\config.json`
//!
//! ## デバイスの識別
//! Raw Input API のデバイスパス（instance_id）をキーとして使用。
//! 同一 VID/PID でも物理的に異なるデバイスは別パスを持つため個体を区別できる。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// デバイスごとの個別設定
#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceConfig {
    /// デバイスのインスタンスID（Raw Input のデバイスパス）
    pub instance_id: String,
    /// ユーザーが設定した表示名（例: "仕事用マウス"）
    pub display_name: String,
    /// カーソル速度（1〜20）
    pub speed: i32,
    /// 監視対象かどうか（false: ListView 非表示、WM_INPUT 無視）
    #[serde(default = "default_monitored")]
    pub monitored: bool,
    /// ポインターの精度を高める（マウス加速）の有効/無効
    #[serde(default = "default_enhance_pointer_precision")]
    pub enhance_pointer_precision: bool,
    /// マウスの主と副のボタンを入れ替えるかどうか
    #[serde(default = "default_swap_buttons")]
    pub swap_buttons: bool,
    /// ホイールのスクロール行数（1〜20）
    #[serde(default = "default_wheel_scroll_lines")]
    pub wheel_scroll_lines: i32,
}

fn default_monitored() -> bool { true }
fn default_enhance_pointer_precision() -> bool { true }
fn default_swap_buttons() -> bool { false }
fn default_wheel_scroll_lines() -> i32 { 3 }

/// アプリケーション全体の設定
#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub devices: Vec<DeviceConfig>,
}

impl AppConfig {
    /// instance_id で検索（不変参照）
    pub fn find_by_instance_id(&self, instance_id: &str) -> Option<&DeviceConfig> {
        self.devices.iter().find(|d| d.instance_id == instance_id)
    }

    /// instance_id で検索（可変参照）
    pub fn find_by_instance_id_mut(&mut self, instance_id: &str) -> Option<&mut DeviceConfig> {
        self.devices.iter_mut().find(|d| d.instance_id == instance_id)
    }
}

fn config_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_default();
    PathBuf::from(appdata)
        .join("multi-mouse-tool")
        .join("config.json")
}

/// 設定ファイルを読み込む（ファイル未存在・パース失敗時はデフォルト値）
pub fn load() -> AppConfig {
    let path = config_path();
    let Ok(content) = fs::read_to_string(&path) else {
        return AppConfig::default();
    };
    serde_json::from_str(&content).unwrap_or_default()
}

/// 設定ファイルに保存する（ディレクトリ未存在時は自動作成）
pub fn save(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("ディレクトリ作成失敗: {}", e))?;
    }
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("JSON変換失敗: {}", e))?;
    fs::write(&path, json)
        .map_err(|e| format!("ファイル書き込み失敗: {}", e))?;
    Ok(())
}
