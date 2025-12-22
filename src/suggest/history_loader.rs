use std::fs;
use std::path::PathBuf;

use crate::history::History;

/// 履歴ファイルのパスを解決する。
///
/// macOSの標準である `~/.zsh_history` を前提にする。
pub(crate) fn resolve_history_path() -> Result<PathBuf, String> {
    History::find_history_file().ok_or("history file not found (~/.zsh_history)".to_string())
}

/// 履歴ファイルを読み込み、文字列として返す。
///
/// 非UTF-8文字が混ざる可能性があるため、損失変換で読み込む。
pub(crate) fn load_history_contents(path: &PathBuf) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|err| format!("failed to read history file: {err}"))?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

/// 履歴全体から直近 `limit` 件のコマンドを抽出する。
///
/// 空の結果になった場合は、上位でエラーとして扱う。
pub(crate) fn collect_recent_commands(contents: &str, limit: usize) -> Result<Vec<String>, String> {
    let recent = History::recent_commands(contents, limit);
    if recent.is_empty() {
        Err("no commands found in recent history".to_string())
    } else {
        Ok(recent)
    }
}
