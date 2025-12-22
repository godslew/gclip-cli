use crate::registry::Registry;
use crate::selection;
use std::path::Path;

/// 登録済みコマンドを削除する機能。
///
/// `gclip --rm "query"` で部分一致検索し、選択して削除する。
pub fn run(query: &str) -> Result<(), String> {
    let matches = Registry::search_commands(query)?;
    ensure_matches(&matches)?;
    print_matches(query, &matches);

    let selection =
        selection::prompt_single_selection(matches.len(), "Select command to remove")?;
    handle_selection(&matches, selection)?;
    Ok(())
}

/// 検索結果が空でないことを確認する。
///
/// 一致がない場合はエラーにして終了する。
fn ensure_matches(matches: &[String]) -> Result<(), String> {
    if matches.is_empty() {
        Err("no registered commands match the query".to_string())
    } else {
        Ok(())
    }
}

/// 検索結果を標準エラーへ表示する。
///
/// 標準出力は結果メッセージのために空けておく。
fn print_matches(query: &str, matches: &[String]) {
    eprintln!("Matches for \"{query}\":");
    for (index, command) in matches.iter().enumerate() {
        eprintln!("{:>2}. {}", index + 1, command);
    }
}

/// 選択結果に応じて削除処理を行う。
fn handle_selection(matches: &[String], selection: Option<usize>) -> Result<(), String> {
    let Some(index) = selection else {
        print_cancelled();
        return Ok(());
    };

    let command = command_at_index(matches, index)?;
    let (registry_path, removed) = Registry::remove_command(command)?;
    print_result(command, removed, &registry_path);
    Ok(())
}

/// 選択されたコマンドを取得する。
///
/// 範囲外の場合はエラーにする。
fn command_at_index(matches: &[String], index: usize) -> Result<&str, String> {
    matches
        .get(index - 1)
        .map(|command| command.as_str())
        .ok_or_else(|| format!("out of range: {index}"))
}

/// 削除結果を標準出力へ表示する。
///
/// 削除件数と登録先を明示する。
fn print_result(command: &str, removed: usize, registry_path: &Path) {
    if removed == 0 {
        println!("Not found: \"{command}\"");
    } else {
        println!("Removed \"{command}\" from {}", registry_path.display());
    }
}

/// キャンセル時のメッセージを標準エラーに出力する。
fn print_cancelled() {
    eprintln!("Cancelled.");
}
