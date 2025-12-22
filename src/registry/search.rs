use std::path::PathBuf;

use super::{io, path, RegisteredCommands};

/// 登録済みコマンドから検索する。
///
/// 部分一致で検索し、入力順を保ったまま結果を返す。
pub(super) fn search_commands(query: &str) -> Result<Vec<String>, String> {
    let normalized = normalize_query(query)?;
    let registry_path = resolve_registry_path()?;
    let registered = io::load_registry(&registry_path)?;
    Ok(filter_commands(&registered, &normalized))
}

/// クエリ文字列を正規化する。
///
/// 前後の空白を除去し、空の場合はエラーにする。
pub(super) fn normalize_query(query: &str) -> Result<String, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        Err("query is empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// 登録済みコマンドを絞り込む。
///
/// - 部分一致で検索する。
/// - 入力の順序は保持する。
pub(super) fn filter_commands(
    registered: &RegisteredCommands,
    query: &str,
) -> Vec<String> {
    registered
        .commands
        .iter()
        .filter(|command| command.contains(query))
        .cloned()
        .collect()
}

/// 登録ファイルのパスを解決する。
///
/// `~/.gclip/registered.toml` を前提にする。
fn resolve_registry_path() -> Result<PathBuf, String> {
    let data_dir = path::data_dir().ok_or("HOME not set")?;
    Ok(path::registry_path(&data_dir))
}
