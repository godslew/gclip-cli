use super::super::selection::parse_selection_input;

#[test]
fn empty_input_returns_empty_vec() {
    // 何も選択されていない場合は、登録処理をスキップできるよう空配列を返す。
    let result = parse_selection_input("", 5).expect("empty input should be ok");
    assert!(result.is_empty());
}

#[test]
fn all_selects_all() {
    // all指定で上限までのインデックスが生成されることを確認する。
    let result = parse_selection_input("all", 3).expect("all should be ok");
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn supports_comma_separated_input() {
    // 代表的な選択形式として1,3の指定が機能することを確認する。
    let result = parse_selection_input("1,3", 3).expect("comma selection should be ok");
    assert_eq!(result, vec![1, 3]);
}

#[test]
fn supports_range_input() {
    // 1-3のような範囲指定が展開されることを確認する。
    let result = parse_selection_input("1-3", 3).expect("range selection should be ok");
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn deduplicates_indices() {
    // 同じ番号を複数回指定しても重複しないことを確認する。
    let result = parse_selection_input("2,2", 3).expect("duplicate selection should be ok");
    assert_eq!(result, vec![2]);
}

#[test]
fn out_of_range_is_error() {
    // 0や上限超過の番号は無効として扱う。
    assert!(parse_selection_input("0", 3).is_err());
    assert!(parse_selection_input("4", 3).is_err());
}

#[test]
fn reverse_range_is_error() {
    // start > end の範囲指定は無効として扱う。
    assert!(parse_selection_input("3-1", 3).is_err());
}

#[test]
fn non_numeric_is_error() {
    // 数値に変換できない入力は無効として扱う。
    assert!(parse_selection_input("a", 3).is_err());
}
