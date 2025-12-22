use std::collections::HashMap;

use super::types::{CommandStats, Recommendation};

/// 直近履歴から推薦候補を構築する。
///
/// 出現回数と最終出現位置を記録し、安定した並び替えに利用する。
pub(crate) fn build_recommendations(recent: &[String]) -> Vec<Recommendation> {
    let stats = count_commands(recent);
    let mut recommendations = to_recommendations(stats);
    sort_recommendations(&mut recommendations);
    recommendations
}

/// 直近履歴を走査して出現回数と最終出現位置を集計する。
fn count_commands(recent: &[String]) -> HashMap<String, CommandStats> {
    let mut counts: HashMap<String, CommandStats> = HashMap::new();
    for (idx, command) in recent.iter().enumerate() {
        let entry = counts.entry(command.clone()).or_default();
        entry.count += 1;
        entry.last_seen = idx;
    }
    counts
}

/// 集計結果を推薦候補の一覧に変換する。
fn to_recommendations(stats: HashMap<String, CommandStats>) -> Vec<Recommendation> {
    stats
        .into_iter()
        .map(|(command, stat)| Recommendation {
            command,
            count: stat.count,
            last_seen: stat.last_seen,
        })
        .collect()
}

/// 推薦候補を優先順位に従って並び替える。
///
/// 優先順位は「回数(降順) -> 最近性(降順) -> 文字列(昇順)」。
fn sort_recommendations(recommendations: &mut [Recommendation]) {
    recommendations.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| b.last_seen.cmp(&a.last_seen))
            .then_with(|| a.command.cmp(&b.command))
    });
}

/// 上位 `max` 件に絞り込む。
///
/// `Recommendation` は所有権ごと受け取り、必要分だけ残して返す。
pub(crate) fn select_top(mut recommendations: Vec<Recommendation>, max: usize) -> Vec<Recommendation> {
    if recommendations.len() > max {
        recommendations.truncate(max);
    }
    recommendations
}
