use super::super::parse_selection_input;

#[test]
fn empty_input_returns_none() {
    // 空入力はキャンセル扱いとしてNoneになる。
    let result = parse_selection_input("", 3).expect("empty input should be ok");
    assert!(result.is_none());
}

#[test]
fn valid_number_returns_index() {
    // 有効な番号はSomeで返ることを確認する。
    let result = parse_selection_input("2", 3).expect("valid input should be ok");
    assert_eq!(result, Some(2));
}

#[test]
fn out_of_range_is_error() {
    // 範囲外の番号はエラーにする。
    assert!(parse_selection_input("0", 3).is_err());
    assert!(parse_selection_input("4", 3).is_err());
}

#[test]
fn non_numeric_is_error() {
    // 数値に変換できない入力は無効として扱う。
    assert!(parse_selection_input("a", 3).is_err());
}
