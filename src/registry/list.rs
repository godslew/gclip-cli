use std::path::PathBuf;

use super::{io, path};

/// 登録済みコマンドを一覧で返す。
///
/// 登録ファイルがない場合は空配列を返す。
pub(super) fn list_commands() -> Result<Vec<String>, String> {
    let registry_path = resolve_registry_path()?;
    let registered = io::load_registry(&registry_path)?;
    Ok(registered.commands)
}

/// 登録ファイルのパスを解決する。
///
/// `~/.gclip/registered.toml` を前提にする。
fn resolve_registry_path() -> Result<PathBuf, String> {
    let data_dir = path::data_dir().ok_or("HOME not set")?;
    Ok(path::registry_path(&data_dir))
}
