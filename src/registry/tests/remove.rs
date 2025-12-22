use super::super::remove::{normalize_command, remove_matching};
use super::super::RegisteredCommands;

#[test]
fn normalize_command_trims_whitespace() {
    // 前後の空白が除去されることを確認する。
    let normalized = normalize_command("  git status  ").expect("should be valid");
    assert_eq!(normalized, "git status");
}

#[test]
fn normalize_command_rejects_empty() {
    // 空文字列は削除対象にならないためエラーにする。
    assert!(normalize_command("   ").is_err());
}

#[test]
fn remove_matching_removes_only_exact_match() {
    // 完全一致だけが削除されることを確認する。
    let mut registered = RegisteredCommands {
        commands: vec!["git status".to_string(), "git".to_string()],
    };
    let removed = remove_matching(&mut registered, "git");
    assert_eq!(removed, 1);
    assert_eq!(
        registered.commands,
        vec!["git status".to_string()]
    );
}
