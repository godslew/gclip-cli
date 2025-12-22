mod io;
mod merge;
mod path;
mod search;
mod add;
mod list;
mod remove;
mod doctor;
mod recent;

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

    /// 1件のコマンド文字列を登録する。
    ///
    /// 空文字列は拒否し、余分な空白は除去する。
    pub fn add_command(command: &str) -> Result<(PathBuf, usize), String> {
        add::add_command(command)
    }

    /// 登録済みコマンドから検索する。
    ///
    /// 部分一致で検索し、入力順を保ったまま結果を返す。
    pub fn search_commands(query: &str) -> Result<Vec<String>, String> {
        search::search_commands(query)
    }

    /// 登録済みコマンドを一覧で返す。
    ///
    /// 登録ファイルがない場合は空配列を返す。
    pub fn list_commands() -> Result<Vec<String>, String> {
        list::list_commands()
    }

    /// 登録済みコマンドを削除する。
    ///
    /// 前後の空白を除去した上で、完全一致で削除する。
    pub fn remove_command(command: &str) -> Result<(PathBuf, usize), String> {
        remove::remove_command(command)
    }

    /// 直近使用コマンドを記録する。
    ///
    /// 先頭に追加し、重複は除去する。
    pub fn record_recent(command: &str) -> Result<(), String> {
        recent::record_recent(command)
    }

    /// 直近使用コマンドの一覧を返す。
    ///
    /// 保存ファイルがない場合は空配列を返す。
    pub fn recent_commands(limit: usize) -> Result<Vec<String>, String> {
        recent::recent_commands(limit)
    }

    /// 設定/保存場所の確認を行う。
    ///
    /// 現在のパスと整合性の診断結果を返す。
    pub fn doctor_report() -> Result<crate::doctor::DoctorReport, String> {
        doctor::doctor_report()
    }
}

#[cfg(test)]
mod tests;
