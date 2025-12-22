use super::super::search::{filter_commands, normalize_query};
use super::super::RegisteredCommands;

#[test]
fn normalize_query_trims_whitespace() {
    // 前後の空白が除去されることを確認する。
    let normalized = normalize_query("  git ").expect("query should be valid");
    assert_eq!(normalized, "git");
}

#[test]
fn normalize_query_rejects_empty() {
    // 空文字列は検索対象にならないためエラーにする。
    assert!(normalize_query("   ").is_err());
}

#[test]
fn filter_commands_matches_substring_in_order() {
    // 部分一致の結果が登録順を維持して返ることを確認する。
    let registered = RegisteredCommands {
        commands: vec![
            "git status".to_string(),
            "ls -la".to_string(),
            "git commit -m test".to_string(),
        ],
    };
    let results = filter_commands(&registered, "git");
    assert_eq!(
        results,
        vec!["git status".to_string(), "git commit -m test".to_string()]
    );
}

#[test]
fn filter_commands_returns_empty_when_no_match() {
    // 一致がない場合は空配列を返す。
    let registered = RegisteredCommands {
        commands: vec!["ls -la".to_string()],
    };
    let results = filter_commands(&registered, "git");
    assert!(results.is_empty());
}
