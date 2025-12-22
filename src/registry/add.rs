use std::path::PathBuf;

use super::Registry;

/// 1件のコマンド文字列を登録する。
///
/// 入力を正規化してから既存登録処理に委譲する。
pub(super) fn add_command(command: &str) -> Result<(PathBuf, usize), String> {
    let normalized = normalize_command(command)?;
    let commands = build_single_command(normalized);
    Registry::register_commands(&commands)
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

/// 単一コマンドを登録用の配列に変換する。
///
/// 既存の登録処理が `&[String]` を受け取るため、
/// ここで `Vec<String>` にまとめる。
fn build_single_command(command: String) -> Vec<String> {
    vec![command]
}
