use super::super::recommend::{build_recommendations, select_top};

#[test]
fn sorts_by_count_then_recency() {
    // 同じ回数なら最終出現位置が新しいものが先に来ることを確認する。
    let recent = vec![
        "ls".to_string(),
        "git status".to_string(),
        "ls".to_string(),
        "make".to_string(),
        "git status".to_string(),
    ];
    let recommendations = build_recommendations(&recent);
    assert_eq!(recommendations[0].command, "git status");
    assert_eq!(recommendations[1].command, "ls");
    assert_eq!(recommendations[2].command, "make");
}

#[test]
fn returns_all_when_under_limit() {
    // 上限を超えない場合はそのままの件数で返ることを確認する。
    let recent = vec!["a".to_string(), "b".to_string()];
    let recommendations = build_recommendations(&recent);
    let top = select_top(recommendations, 10);
    assert_eq!(top.len(), 2);
}

#[test]
fn returns_empty_when_limit_zero() {
    // 0指定なら常に空配列を返すことを確認する。
    let recent = vec!["a".to_string(), "b".to_string()];
    let recommendations = build_recommendations(&recent);
    let top = select_top(recommendations, 0);
    assert!(top.is_empty());
}
