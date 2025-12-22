use super::super::parse::parse_history_line;

#[test]
fn ignores_empty_lines() {
    // 空行や空白のみの行は履歴のコマンドとして扱わない。
    assert_eq!(parse_history_line(""), None);
    assert_eq!(parse_history_line("   "), None);
    assert_eq!(parse_history_line("\t"), None);
}

#[test]
fn parses_zsh_extended_format() {
    // `;` 以降のコマンドだけを取り出せることを確認する。
    let line = ": 1700000000:0;ls -la";
    assert_eq!(parse_history_line(line), Some("ls -la".to_string()));
}

#[test]
fn ignores_empty_zsh_extended_command() {
    // `;` の後が空のケースは履歴として扱わない。
    let line = ": 1700000000:0;";
    assert_eq!(parse_history_line(line), None);
}

#[test]
fn accepts_plain_format() {
    // zsh拡張形式に該当しない場合は、その行をコマンドとして扱う。
    let line = "git status";
    assert_eq!(parse_history_line(line), Some("git status".to_string()));
}
