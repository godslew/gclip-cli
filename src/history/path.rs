use std::env;
use std::path::{Path, PathBuf};

/// 履歴ファイルのパスを解決する。
///
/// macOSの標準構成として `~/.zsh_history` のみを探索対象とする。
pub(crate) fn find_history_file() -> Option<PathBuf> {
    let home = home_dir()?;
    let path = history_path_from_home(&home);
    path_if_exists(path)
}

/// `HOME` 環境変数からホームディレクトリを取得する。
///
/// 取得できない場合は `None` を返し、上位で処理を中断する。
fn home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

/// ホームディレクトリから zsh 履歴のパスを組み立てる。
///
/// macOSの標準構成を前提に、`~/.zsh_history` を返す。
fn history_path_from_home(home: &Path) -> PathBuf {
    home.join(".zsh_history")
}

/// パスが存在する場合のみ `Some` を返す。
///
/// 存在しない場合は `None` とし、呼び出し側でエラー扱いにする。
fn path_if_exists(path: PathBuf) -> Option<PathBuf> {
    if path.exists() {
        Some(path)
    } else {
        None
    }
}
