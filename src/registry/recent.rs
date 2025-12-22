use std::path::PathBuf;

use super::{path, RegisteredCommands};

const RECENT_LIMIT: usize = 50;

/// 直近使用コマンドの一覧を返す。
///
/// 保存ファイルがない場合は空配列を返す。
pub(super) fn recent_commands(limit: usize) -> Result<Vec<String>, String> {
    let recent_path = resolve_recent_path()?;
    let recent = load_recent(&recent_path)?;
    Ok(recent.commands.into_iter().take(limit).collect())
}

/// 直近使用コマンドとして記録する。
///
/// - 先頭に追加する
/// - 既存の同一コマンドは除去する
/// - 上限を超えた分は切り捨てる
pub(super) fn record_recent(command: &str) -> Result<(), String> {
    let normalized = normalize_command(command)?;
    let recent_path = resolve_recent_path()?;
    let mut recent = load_recent(&recent_path)?;

    recent.commands.retain(|item| item != &normalized);
    recent.commands.insert(0, normalized);
    if recent.commands.len() > RECENT_LIMIT {
        recent.commands.truncate(RECENT_LIMIT);
    }

    save_recent(&recent_path, &recent)?;
    Ok(())
}

/// コマンド文字列の正規化を行う。
///
/// - 前後の空白を除去する
/// - 空文字列はエラーにする
fn normalize_command(command: &str) -> Result<String, String> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        Err("command is empty".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

/// 直近使用コマンドのファイルを読み込む。
fn load_recent(path: &PathBuf) -> Result<RegisteredCommands, String> {
    if !path.exists() {
        return Ok(RegisteredCommands::default());
    }
    let contents =
        std::fs::read_to_string(path).map_err(|err| format!("failed to read recent file: {err}"))?;
    if contents.trim().is_empty() {
        return Ok(RegisteredCommands::default());
    }
    toml::from_str::<RegisteredCommands>(&contents)
        .map_err(|err| format!("failed to parse recent file: {err}"))
}

/// 直近使用コマンドのファイルを書き込む。
fn save_recent(path: &PathBuf, recent: &RegisteredCommands) -> Result<(), String> {
    let mut serialized = toml::to_string_pretty(recent).map_err(|err| format!("{err}"))?;
    if !serialized.ends_with('\n') {
        serialized.push('\n');
    }
    std::fs::write(path, serialized).map_err(|err| format!("failed to write recent file: {err}"))
}

/// 直近使用コマンドの保存先パスを解決する。
fn resolve_recent_path() -> Result<PathBuf, String> {
    let data_dir = path::data_dir().ok_or("HOME not set")?;
    path::ensure_dir(&data_dir)?;
    Ok(path::recent_path(&data_dir))
}
