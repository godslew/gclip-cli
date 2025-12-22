mod parse;
mod path;
mod recent;

use std::path::PathBuf;

/// シェル履歴の読み取り・解析に関する処理をまとめる。
///
/// 呼び出し側は `History` の公開メソッドだけを使うことで、
/// 履歴取得の手順を意識せずに利用できる。
pub struct History;

impl History {
    /// 履歴ファイルのパスを返す（macOS前提）。
    ///
    /// macOSの標準シェルはzshなので `~/.zsh_history` のみを対象とする。
    pub fn find_history_file() -> Option<PathBuf> {
        path::find_history_file()
    }

    /// 履歴ファイル全体の文字列から、直近 `limit` 件のコマンドを抽出する。
    ///
    /// - 行単位で解析し、空行は除外する。
    /// - 解析結果が `limit` を超える場合は末尾の `limit` 件だけ返す。
    pub fn recent_commands(contents: &str, limit: usize) -> Vec<String> {
        recent::recent_commands(contents, limit)
    }
}

#[cfg(test)]
mod tests;
