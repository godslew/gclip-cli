use super::support::TestHome;
use super::super::recent::{record_recent, recent_commands};

#[test]
fn record_recent_moves_command_to_front() {
    // 同じコマンドがあれば先頭に移動し、重複しないことを確認する。
    let _home = TestHome::new();
    record_recent("ls").expect("record should succeed");
    record_recent("pwd").expect("record should succeed");
    record_recent("ls").expect("record should succeed");

    let recent = recent_commands(10).expect("recent should succeed");
    assert_eq!(recent, vec!["ls".to_string(), "pwd".to_string()]);
}

#[test]
fn recent_commands_respects_limit() {
    // 取得件数の上限が守られることを確認する。
    let _home = TestHome::new();
    record_recent("one").expect("record should succeed");
    record_recent("two").expect("record should succeed");
    record_recent("three").expect("record should succeed");

    let recent = recent_commands(2).expect("recent should succeed");
    assert_eq!(recent, vec!["three".to_string(), "two".to_string()]);
}
