use std::path::PathBuf;

use crate::registry::Registry;

/// 選択されたコマンドのみを登録ファイルへ追記する。
///
/// Registryの詳細を呼び出し側から隠し、suggest機能の責務を明確にする。
pub(crate) fn register_selected_commands(commands: &[String]) -> Result<(PathBuf, usize), String> {
    Registry::register_commands(commands)
}
