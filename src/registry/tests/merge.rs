use super::super::merge::merge_commands;
use super::super::RegisteredCommands;

#[test]
fn adds_new_commands() {
    // 既存にないコマンドだけが追加されることを確認する。
    let mut registered = RegisteredCommands {
        commands: vec!["ls".to_string()],
    };
    let added = merge_commands(&mut registered, &["pwd".to_string()]);
    assert_eq!(added, 1);
    assert_eq!(registered.commands, vec!["ls".to_string(), "pwd".to_string()]);
}

#[test]
fn does_not_add_duplicates() {
    // 既に登録済みのコマンドは追加されないことを確認する。
    let mut registered = RegisteredCommands {
        commands: vec!["ls".to_string()],
    };
    let added = merge_commands(&mut registered, &["ls".to_string()]);
    assert_eq!(added, 0);
    assert_eq!(registered.commands, vec!["ls".to_string()]);
}

#[test]
fn no_change_on_empty_input() {
    // 入力が空の場合は何も変更されないことを確認する。
    let mut registered = RegisteredCommands {
        commands: vec!["ls".to_string()],
    };
    let added = merge_commands(&mut registered, &[]);
    assert_eq!(added, 0);
    assert_eq!(registered.commands, vec!["ls".to_string()]);
}

#[test]
fn ignores_duplicate_inputs() {
    // 入力側に同じコマンドが複数あっても追加は1回になる。
    let mut registered = RegisteredCommands {
        commands: vec!["ls".to_string()],
    };
    let added = merge_commands(
        &mut registered,
        &["pwd".to_string(), "pwd".to_string()],
    );
    assert_eq!(added, 1);
    assert_eq!(registered.commands, vec!["ls".to_string(), "pwd".to_string()]);
}
