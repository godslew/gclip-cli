use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// 登録ファイルの保存先ディレクトリを解決する。
///
/// すべての設定ファイルを `~/.gclip` 配下に集約する方針とする。
pub(crate) fn data_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(|home| Path::new(&home).join(".gclip"))
}

/// 登録ファイルの絶対パスを組み立てる。
///
/// `registered.toml` を `~/.gclip` 直下に配置する。
pub(crate) fn registry_path(data_dir: &Path) -> PathBuf {
    data_dir.join("registered.toml")
}

/// 直近使用コマンドの保存先パスを組み立てる。
///
/// `recent.toml` を `~/.gclip` 直下に配置する。
pub(crate) fn recent_path(data_dir: &Path) -> PathBuf {
    data_dir.join("recent.toml")
}

/// 設定ディレクトリがなければ作成する。
///
/// 既に存在する場合は何もしない。
pub(crate) fn ensure_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|err| format!("failed to create data dir: {err}"))
}
