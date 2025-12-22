/// 履歴の1行をパースしてコマンド文字列を返す。
///
/// 対応パターン:
/// - zsh拡張形式 `: 1700000000:0;ls -la`
/// - 通常形式 `ls -la`
///
/// 解析不能または空行の場合は `None` を返す。
pub(super) fn parse_history_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    // zsh拡張形式の場合は結果をそのまま返し、空ならNoneとする。
    if trimmed.starts_with(": ") {
        return parse_zsh_extended_line(trimmed);
    }

    // 通常形式はそのままコマンドとして扱う。
    parse_plain_line(trimmed)
}

/// zshの拡張履歴形式 `: 1700000000:0;cmd` を解析する。
///
/// `;` 以降をコマンドとして返し、空なら `None` を返す。
fn parse_zsh_extended_line(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with(": ") {
        return None;
    }

    let pos = trimmed.find(';')?;
    let command = trimmed[pos + 1..].trim();
    if command.is_empty() {
        None
    } else {
        Some(command.to_string())
    }
}

/// 通常の履歴行をそのままコマンドとして扱う。
///
/// すでに空行は除外済みなので、そのまま `String` 化する。
fn parse_plain_line(trimmed: &str) -> Option<String> {
    Some(trimmed.to_string())
}
