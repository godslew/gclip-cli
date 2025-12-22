use std::collections::HashSet;
use std::io::{self, Write};

use super::types::Recommendation;

/// 推薦結果が空でないことを確認する。
///
/// ここで空なら、その後の選択処理が意味を持たないためエラーにする。
pub(crate) fn ensure_recommendations(recommendations: &[Recommendation]) -> Result<(), String> {
    if recommendations.is_empty() {
        Err("no commands to recommend".to_string())
    } else {
        Ok(())
    }
}

/// 対話的に登録対象を選択してもらう。
///
/// 不正な入力があった場合は再入力を促す。
pub(crate) fn prompt_selection(max: usize) -> Result<Vec<usize>, String> {
    loop {
        print_selection_prompt(max)?;
        let input = read_input_line()?;
        match parse_selection_input(&input, max) {
            Ok(indices) => return Ok(indices),
            Err(err) => {
                eprintln!("Invalid selection: {err}");
            }
        }
    }
}

/// 選択されたインデックスから、登録対象のコマンド一覧を作る。
///
/// インデックスは1始まりなので、0始まりの配列に変換して取り出す。
pub(crate) fn select_commands(recommendations: &[Recommendation], indices: &[usize]) -> Vec<String> {
    indices
        .iter()
        .filter_map(|index| recommendations.get(index - 1))
        .map(|rec| rec.command.clone())
        .collect()
}

/// 選択プロンプトを表示し、標準出力をフラッシュする。
///
/// 入力待ちの前に必ずフラッシュして表示漏れを防ぐ。
fn print_selection_prompt(max: usize) -> Result<(), String> {
    print!(
        "Select commands to register (1-{}, e.g. 1,3 or 1-3, 'all', empty to cancel): ",
        max
    );
    io::stdout()
        .flush()
        .map_err(|err| format!("failed to flush stdout: {err}"))
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

/// 入力文字列を解析して、選択されたインデックスを返す。
///
/// - 空入力: 何も選ばない（空配列）
/// - `all`: 全件選択
/// - `1,3,5` または `1-3` のような形式に対応
pub(crate) fn parse_selection_input(input: &str, max: usize) -> Result<Vec<usize>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if trimmed.eq_ignore_ascii_case("all") {
        return Ok((1..=max).collect());
    }

    let tokens = tokenize_selection(trimmed);
    let mut indices = Vec::new();
    let mut seen = HashSet::new();
    for token in tokens {
        add_indices_from_token(&token, max, &mut indices, &mut seen)?;
    }

    if indices.is_empty() {
        Err("no valid selection".to_string())
    } else {
        Ok(indices)
    }
}

/// カンマと空白で分割し、選択トークン一覧を返す。
fn tokenize_selection(input: &str) -> Vec<String> {
    input
        .split(',')
        .flat_map(|part| part.split_whitespace())
        .filter(|token| !token.is_empty())
        .map(|token| token.to_string())
        .collect()
}

/// 1トークンからインデックスを追加する。
///
/// - 単一番号
/// - `start-end` の範囲指定
fn add_indices_from_token(
    token: &str,
    max: usize,
    indices: &mut Vec<usize>,
    seen: &mut HashSet<usize>,
) -> Result<(), String> {
    if let Some((start, end)) = parse_range(token)? {
        for idx in start..=end {
            push_index(idx, max, indices, seen)?;
        }
        return Ok(());
    }

    let index = parse_single_index(token)?;
    push_index(index, max, indices, seen)
}

/// `start-end` 形式の範囲指定を解析する。
///
/// 該当しない場合は `Ok(None)` を返す。
fn parse_range(token: &str) -> Result<Option<(usize, usize)>, String> {
    let (start_str, end_str) = match token.split_once('-') {
        Some(parts) => parts,
        None => return Ok(None),
    };

    let start = parse_single_index(start_str)?;
    let end = parse_single_index(end_str)?;
    if start > end {
        return Err(format!("invalid range: {token}"));
    }
    Ok(Some((start, end)))
}

/// 単一の番号を解析する。
///
/// 数値でない場合はエラーにする。
fn parse_single_index(token: &str) -> Result<usize, String> {
    token
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("not a number: {token}"))
}

/// 選択されたインデックスを追加する。
///
/// 範囲外や重複は許容せず、エラーまたは無視で扱う。
fn push_index(
    index: usize,
    max: usize,
    indices: &mut Vec<usize>,
    seen: &mut HashSet<usize>,
) -> Result<(), String> {
    if index == 0 || index > max {
        return Err(format!("out of range: {index}"));
    }
    if seen.insert(index) {
        indices.push(index);
    }
    Ok(())
}
