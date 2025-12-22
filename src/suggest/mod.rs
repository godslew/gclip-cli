mod config;
mod history_loader;
mod output;
mod recommend;
mod registry_ops;
mod selection;
mod types;

use config::{HISTORY_SAMPLE_SIZE, MAX_RECOMMENDATIONS};

/// `--suggest` 機能の実行本体。
///
/// 1. 履歴ファイルを特定して読み込む
/// 2. 直近100件のコマンドを抽出
/// 3. 頻度順に並べて上位10件を推薦
/// 4. 推薦結果を選択して登録する
pub fn run() -> Result<(), String> {
    let history_path = history_loader::resolve_history_path()?;
    let contents = history_loader::load_history_contents(&history_path)?;
    let recent = history_loader::collect_recent_commands(&contents, HISTORY_SAMPLE_SIZE)?;

    let recommendations = recommend::build_recommendations(&recent);
    let top = recommend::select_top(recommendations, MAX_RECOMMENDATIONS);
    selection::ensure_recommendations(&top)?;

    output::print_recommendations(&history_path, &top, HISTORY_SAMPLE_SIZE);

    let selected = selection::prompt_selection(top.len())?;
    if selected.is_empty() {
        output::print_selection_cancelled();
        return Ok(());
    }

    let commands = selection::select_commands(&top, &selected);
    let (registry_path, added) = registry_ops::register_selected_commands(&commands)?;
    output::print_registration_result(added, &registry_path);
    Ok(())
}

#[cfg(test)]
mod tests;
