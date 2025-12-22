use super::support::TestHome;
use super::super::list::list_commands;
use super::super::path::ensure_dir;
use std::fs;

#[test]
fn list_returns_empty_when_file_missing() {
    // 未作成の登録ファイルは空として扱われる。
    let _home = TestHome::new();
    let commands = list_commands().expect("list should succeed");
    assert!(commands.is_empty());
}

#[test]
fn list_returns_commands_in_order() {
    // 登録済みコマンドが順序通りに返ることを確認する。
    let home = TestHome::new();
    let registry_path = home.registry_path();
    // 登録ファイルを書き込むため、親ディレクトリを先に用意する。
    let registry_dir = registry_path
        .parent()
        .expect("registry dir should exist");
    ensure_dir(registry_dir).expect("ensure registry dir should succeed");

    let contents = r#"
commands = ["ls", "pwd"]
"#;
    fs::write(&registry_path, contents.trim_start()).expect("write should succeed");

    let commands = list_commands().expect("list should succeed");
    assert_eq!(commands, vec!["ls".to_string(), "pwd".to_string()]);
}
