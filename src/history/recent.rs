use super::parse;

/// 履歴ファイル全体の文字列から、直近 `limit` 件のコマンドを抽出する。
///
/// - 行単位で解析し、空行や解析不能行は除外する。
/// - 解析結果が `limit` を超える場合は末尾の `limit` 件だけ返す。
pub(super) fn recent_commands(contents: &str, limit: usize) -> Vec<String> {
    let commands = collect_commands(contents);
    limit_recent(commands, limit)
}

/// 履歴ファイル全体からコマンドを抽出する。
///
/// 行単位で `parse_history_line` を呼び出して集約する。
fn collect_commands(contents: &str) -> Vec<String> {
    let mut commands = Vec::new();
    for line in contents.lines() {
        if let Some(command) = parse::parse_history_line(line) {
            commands.push(command);
        }
    }
    commands
}

/// 直近 `limit` 件に絞り込む。
///
/// コマンド件数が少ない場合はそのまま返す。
fn limit_recent(mut commands: Vec<String>, limit: usize) -> Vec<String> {
    if commands.len() > limit {
        commands.split_off(commands.len() - limit)
    } else {
        commands
    }
}
