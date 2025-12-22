use std::path::PathBuf;

use super::{io, path, RegisteredCommands};

/// 登録済みコマンドを削除する。
///
/// 前後の空白を除去した上で、完全一致で削除する。
pub(super) fn remove_command(command: &str) -> Result<(PathBuf, usize), String> {
    let normalized = normalize_command(command)?;
    let registry_path = resolve_registry_path()?;
    let mut registered = io::load_registry(&registry_path)?;
    let removed = remove_matching(&mut registered, &normalized);

    if removed > 0 {
        io::write_registry(&registry_path, &registered)?;
    }

    Ok((registry_path, removed))
}

/// コマンド文字列の正規化を行う。
///
/// - 前後の空白を除去する
/// - 空文字列はエラーにする
pub(super) fn normalize_command(command: &str) -> Result<String, String> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        Err("command is empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// 登録済みリストから完全一致のコマンドを削除する。
///
/// 削除件数を返し、0なら何も削除されていないことを示す。
pub(super) fn remove_matching(registered: &mut RegisteredCommands, command: &str) -> usize {
    let before = registered.commands.len();
    registered.commands.retain(|item| item != command);
    before - registered.commands.len()
}

/// 登録ファイルのパスを解決する。
///
/// `~/.gclip/registered.toml` を前提にする。
fn resolve_registry_path() -> Result<PathBuf, String> {
    let data_dir = path::data_dir().ok_or("HOME not set")?;
    Ok(path::registry_path(&data_dir))
}
