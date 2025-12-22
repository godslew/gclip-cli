/// 推薦結果の1件分を保持する。
///
/// 並び替えに使う情報と出力に必要な情報をまとめて扱う。
#[derive(Debug)]
pub(crate) struct Recommendation {
    pub(crate) command: String,
    pub(crate) count: usize,
    pub(crate) last_seen: usize,
}

/// 集計の途中で使う統計情報。
///
/// HashMapに詰めておき、後続の並び替え処理へ渡す。
#[derive(Debug, Default)]
pub(crate) struct CommandStats {
    pub(crate) count: usize,
    pub(crate) last_seen: usize,
}
