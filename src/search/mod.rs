mod widget;

use crate::registry::Registry;
use std::io::{self, Write};

/// 登録済みコマンドから検索する機能。
///
/// `gclip <QUERY>` で部分一致検索を行う。
pub fn run(query: &str) -> Result<(), String> {
    let matches = Registry::search_commands(query)?;
    ensure_matches(&matches)?;
    print_matches(query, &matches);

    let selection = prompt_selection(matches.len())?;
    handle_selection(&matches, selection)?;
    Ok(())
}

/// zsh用の挿入ウィジェットを出力する。
///
/// 生成されたスクリプトを `.zshrc` で読み込む想定。
pub fn print_zsh_widget() {
    widget::print_zsh_widget();
}

/// セットアップ用のスクリプトを標準出力へ出力する。
///
/// `.zshrc` から評価されることを想定している。
pub fn print_init_script() {
    widget::print_init_script();
}

#[cfg(test)]
mod tests;

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
/// 標準出力は挿入するコマンドのために空けておく。
fn print_matches(query: &str, matches: &[String]) {
    eprintln!("Matches for \"{query}\":");
    for (index, command) in matches.iter().enumerate() {
        eprintln!("{:>2}. {}", index + 1, command);
    }
}

/// 選択プロンプトを表示し、インデックスを取得する。
///
/// 空入力はキャンセル扱いとして `None` を返す。
fn prompt_selection(max: usize) -> Result<Option<usize>, String> {
    loop {
        print_prompt(max)?;
        let input = read_input_line()?;
        match parse_selection_input(&input, max) {
            Ok(selection) => return Ok(selection),
            Err(err) => {
                eprintln!("Invalid selection: {err}");
            }
        }
    }
}

/// 選択プロンプトを標準エラーへ出力し、フラッシュする。
///
/// 標準出力は最終的なコマンド出力のために空けておく。
fn print_prompt(max: usize) -> Result<(), String> {
    eprint!("Select command to insert (1-{}, empty to cancel): ", max);
    io::stderr()
        .flush()
        .map_err(|err| format!("failed to flush stderr: {err}"))
}

/// 標準入力から1行読み取って返す。
///
/// 入力が読めない場合はエラーにする。
fn read_input_line() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|err| format!("failed to read input: {err}"))?;
    Ok(input)
}

/// 選択入力を解析し、インデックスを返す。
///
/// - 空入力: `None`
/// - 数値1〜max: `Some(index)`
pub(super) fn parse_selection_input(input: &str, max: usize) -> Result<Option<usize>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let index = parse_index(trimmed)?;
    if index == 0 || index > max {
        return Err(format!("out of range: {index}"));
    }
    Ok(Some(index))
}

/// 数値の選択インデックスを解析する。
///
/// 数値以外はエラーにする。
fn parse_index(token: &str) -> Result<usize, String> {
    token
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("not a number: {token}"))
}

/// 選択結果に応じて、出力または実行を行う。
fn handle_selection(matches: &[String], selection: Option<usize>) -> Result<(), String> {
    let Some(index) = selection else {
        print_cancelled();
        return Ok(());
    };

    let command = command_at_index(matches, index)?;
    print_selected_command(command);
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

/// 選択されたコマンドを標準出力へ出力する。
///
/// 標準出力はこの1行のみとし、シェル側で扱いやすくする。
fn print_selected_command(command: &str) {
    println!("{command}");
}

/// キャンセル時のメッセージを標準エラーに出力する。
fn print_cancelled() {
    eprintln!("Cancelled.");
}
