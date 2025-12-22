use std::path::PathBuf;

use super::types::Recommendation;

/// 推薦一覧を表示し、選択を促すための出力を行う。
///
/// - 使用した履歴ファイル
/// - 推薦件数と対象件数
/// - 各推薦の順位と出現回数
pub(crate) fn print_recommendations(
    history_path: &PathBuf,
    recommendations: &[Recommendation],
    history_sample_size: usize,
) {
    println!("History file: {}", history_path.display());
    println!(
        "Recommendations (top {} from last {} commands):",
        recommendations.len(),
        history_sample_size
    );
    for (index, rec) in recommendations.iter().enumerate() {
        println!("{:>2}. {} ({}x)", index + 1, rec.command, rec.count);
    }
}

/// ユーザーが選択をキャンセルした場合の表示。
pub(crate) fn print_selection_cancelled() {
    println!("No commands selected. Nothing was registered.");
}

/// 登録結果を標準出力へ表示する。
///
/// 何件追加されたかと登録先ファイルを明示する。
pub(crate) fn print_registration_result(added: usize, registry_path: &PathBuf) {
    println!(
        "Registered {} new command(s) to {}",
        added,
        registry_path.display()
    );
}
