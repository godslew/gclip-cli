use std::fs;
use std::path::Path;

use super::RegisteredCommands;

/// 既存の登録ファイルを読み込み、構造体へ変換する。
///
/// 未作成の場合は空の状態を返す。
pub(crate) fn load_registry(path: &Path) -> Result<RegisteredCommands, String> {
    if !path.exists() {
        return Ok(RegisteredCommands::default());
    }

    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read registry file: {err}"))?;
    if contents.trim().is_empty() {
        return Ok(RegisteredCommands::default());
    }
    toml::from_str::<RegisteredCommands>(&contents)
        .map_err(|err| format!("failed to parse registry file: {err}"))
}

/// 構造体をTOMLに整形して保存する。
///
/// 末尾に改行を付与して、手動編集時の差分が分かりやすい形にする。
pub(crate) fn write_registry(path: &Path, registered: &RegisteredCommands) -> Result<(), String> {
    let mut serialized = toml::to_string_pretty(registered).map_err(|err| format!("{err}"))?;
    if !serialized.ends_with('\n') {
        serialized.push('\n');
    }
    fs::write(path, serialized).map_err(|err| format!("failed to write registry file: {err}"))
}
