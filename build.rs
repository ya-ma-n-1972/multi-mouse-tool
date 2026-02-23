/// ComCtl32 v6 有効化のため、アプリケーションマニフェストを .exe に埋め込む。
/// resource.rc → multi-mouse-tool.exe.manifest → .exe の連携で動作する。
fn main() {
    embed_resource::compile("resource.rc", embed_resource::NONE);
}
