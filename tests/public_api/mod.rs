use gclip_cli::history::History;

#[test]
fn public_api_can_read_history() {
    // 公開APIとしてHistoryが使えることを確認する簡易テスト。
    let contents = ["pwd", "ls"].join("\n");
    let recent = History::recent_commands(&contents, 10);
    assert_eq!(recent, vec!["pwd".to_string(), "ls".to_string()]);
}
