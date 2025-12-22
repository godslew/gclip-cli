mod io;
mod merge;
mod path;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 推薦されたコマンドを「登録済み」として保存するための処理群。
///
/// ここでの登録とは、TOMLファイルの配列に追記することを指す。
pub struct Registry;

/// 登録ファイルのスキーマ。
///
/// TOMLの配列として保存し、編集しやすい形にする。
#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct RegisteredCommands {
    pub(super) commands: Vec<String>,
}

impl Registry {
    /// 推薦されたコマンドを登録ファイルへ追記する。
    ///
    /// - すでに登録済みのコマンドは重複登録しない。
    /// - 追加件数と登録ファイルのパスを返す。
    pub fn register_commands(commands: &[String]) -> Result<(PathBuf, usize), String> {
        let data_dir = path::data_dir().ok_or("HOME not set")?;
        path::ensure_dir(&data_dir)?;
        let registry_path = path::registry_path(&data_dir);

        let mut registered = io::load_registry(&registry_path)?;
        let added = merge::merge_commands(&mut registered, commands);

        if added > 0 {
            io::write_registry(&registry_path, &registered)?;
        }

        Ok((registry_path, added))
    }
}

#[cfg(test)]
mod tests;
