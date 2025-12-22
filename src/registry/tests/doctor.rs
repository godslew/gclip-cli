use super::super::doctor::{count_duplicate_commands, count_empty_commands};

#[test]
fn count_empty_commands_detects_blank() {
    // 空や空白のみのコマンドが数えられることを確認する。
    let commands = vec!["".to_string(), "  ".to_string(), "ls".to_string()];
    assert_eq!(count_empty_commands(&commands), 2);
}

#[test]
fn count_duplicate_commands_detects_duplicates() {
    // 重複したコマンドだけがカウントされることを確認する。
    let commands = vec!["ls".to_string(), "ls".to_string(), "pwd".to_string()];
    assert_eq!(count_duplicate_commands(&commands), 1);
}
