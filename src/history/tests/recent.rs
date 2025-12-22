use super::super::recent::recent_commands;

#[test]
fn truncates_to_recent_limit() {
    // 直近2件だけを残す想定で、末尾から抽出できることを確認する。
    let contents = ["one", "two", "three"].join("\n");
    let recent = recent_commands(&contents, 2);
    assert_eq!(recent, vec!["two".to_string(), "three".to_string()]);
}

#[test]
fn returns_all_when_under_limit() {
    // 履歴件数が上限より少ない場合は、そのまま返す。
    let contents = ["only"].join("\n");
    let recent = recent_commands(&contents, 10);
    assert_eq!(recent, vec!["only".to_string()]);
}

#[test]
fn filters_empty_lines() {
    // 空行を含む場合でもコマンドのみが返ることを確認する。
    let contents = ["git status", "", "ls -la", "   "].join("\n");
    let recent = recent_commands(&contents, 10);
    assert_eq!(
        recent,
        vec!["git status".to_string(), "ls -la".to_string()]
    );
}

#[test]
fn includes_zsh_extended_lines() {
    // zsh拡張形式が混在していても抽出対象になることを確認する。
    let contents = [": 1700000000:0;pwd", "echo done"].join("\n");
    let recent = recent_commands(&contents, 10);
    assert_eq!(recent, vec!["pwd".to_string(), "echo done".to_string()]);
}
