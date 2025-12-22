/// 直近の履歴から何件を対象にするかの上限。
///
/// 多すぎると処理が重くなるため、現状は100件に固定する。
pub(crate) const HISTORY_SAMPLE_SIZE: usize = 100;

/// 推薦として提示する最大件数。
///
/// 画面上で選択しやすい数に抑えるため、現状は10件に固定する。
pub(crate) const MAX_RECOMMENDATIONS: usize = 10;
