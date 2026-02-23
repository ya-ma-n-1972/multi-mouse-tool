//! # Multi Mouse Tool
//!
//! 複数のマウスデバイスを個別に管理・設定するための Windows デスクトップアプリケーション。
//! タスクトレイに常駐し、接続中のマウスデバイスの識別やカーソル速度の調整が可能。
//!
//! ## モジュール構成
//! - [`app`]: メインウィンドウ、トレイ、イベントハンドリングを含むUIモジュール
//! - [`config`]: デバイスごとの設定永続化（JSON）
//! - [`device`]: Raw Input API によるマウスデバイスの列挙・イベント受信
//! - [`mouse_config`]: Windows SystemParametersInfo によるマウス設定の取得・設定

mod app;
mod config;
mod device;
mod mouse_config;

use app::App;
use native_windows_gui::NativeUi;

fn main() {
    native_windows_gui::init().expect("NWGの初期化に失敗");
    // _app: スコープ保持が目的（Drop でイベントハンドラが解除されるため）
    let _app = App::build_ui(App::default()).expect("UIの構築に失敗");
    native_windows_gui::dispatch_thread_events();
}
