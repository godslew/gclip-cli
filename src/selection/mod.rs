use std::io::{self, Write};

/// 単一選択の入力を促し、選択されたインデックスを返す。
///
/// - 空入力はキャンセル扱いとして `None` を返す。
/// - 不正な入力は再入力を促す。
pub(crate) fn prompt_single_selection(max: usize, label: &str) -> Result<Option<usize>, String> {
    loop {
        print_prompt(label, max)?;
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
/// 標準出力は挿入対象のコマンド出力に使うため、混ぜないようにする。
fn print_prompt(label: &str, max: usize) -> Result<(), String> {
    eprint!("{label} (1-{max}, empty to cancel): ");
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

#[cfg(test)]
mod tests;
