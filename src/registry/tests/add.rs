use super::super::add::normalize_command;

#[test]
fn normalize_command_trims_whitespace() {
    // 前後の空白が除去されることを確認する。
    let normalized = normalize_command("  git status  ").expect("should be valid");
    assert_eq!(normalized, "git status");
}

#[test]
fn normalize_command_rejects_empty() {
    // 空文字列は登録対象にならないためエラーにする。
    assert!(normalize_command("   ").is_err());
}

#[test]
fn accepts_long_command() {
    // 長めのパイプ付きコマンドでもそのまま登録できることを確認する。
    let command = r#"gh pr list --author godslew --state all --limit 500 --json number,createdAt,title --search "created:2025-10-01..2025-12-31" 2>/dev/null | jq -r '.[].createdAt' | cut -c1-7 | sort | uniq -c | awk '{print; sum+=$1} END{print sum, "合計"}'"#;
    let normalized = normalize_command(command).expect("long command should be valid");
    assert_eq!(normalized, command);
}
