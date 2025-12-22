use std::collections::HashSet;

use crate::doctor::DoctorReport;

use super::{path, RegisteredCommands};

/// 設定/保存場所の確認を行う。
///
/// 現在のパスと整合性の診断結果を返す。
pub(super) fn doctor_report() -> Result<DoctorReport, String> {
    let data_dir = path::data_dir().ok_or("HOME not set")?;
    let registry_path = path::registry_path(&data_dir);

    let mut report = DoctorReport {
        data_dir: data_dir.clone(),
        registry_path: registry_path.clone(),
        data_dir_exists: data_dir.exists(),
        registry_exists: registry_path.exists(),
        registry_readable: false,
        registry_valid: false,
        command_count: 0,
        empty_commands: 0,
        duplicate_commands: 0,
        errors: Vec::new(),
    };

    if !report.registry_exists {
        report.registry_readable = true;
        report.registry_valid = true;
        return Ok(report);
    }

    let contents = match std::fs::read_to_string(&registry_path) {
        Ok(contents) => {
            report.registry_readable = true;
            contents
        }
        Err(err) => {
            report
                .errors
                .push(format!("failed to read registry file: {err}"));
            return Ok(report);
        }
    };

    if contents.trim().is_empty() {
        report.registry_valid = true;
        return Ok(report);
    }

    let registered = match parse_registry(&contents) {
        Ok(registered) => {
            report.registry_valid = true;
            registered
        }
        Err(err) => {
            report.errors.push(err);
            return Ok(report);
        }
    };

    apply_command_stats(&mut report, &registered);
    Ok(report)
}

/// TOML文字列を登録コマンドに変換する。
fn parse_registry(contents: &str) -> Result<RegisteredCommands, String> {
    toml::from_str::<RegisteredCommands>(contents)
        .map_err(|err| format!("failed to parse registry file: {err}"))
}

/// コマンド数や重複数などの統計をreportへ反映する。
fn apply_command_stats(report: &mut DoctorReport, registered: &RegisteredCommands) {
    report.command_count = registered.commands.len();
    report.empty_commands = count_empty_commands(&registered.commands);
    report.duplicate_commands = count_duplicate_commands(&registered.commands);
}

/// 空コマンドの数を数える。
pub(super) fn count_empty_commands(commands: &[String]) -> usize {
    commands.iter().filter(|cmd| cmd.trim().is_empty()).count()
}

/// 重複コマンドの数を数える。
pub(super) fn count_duplicate_commands(commands: &[String]) -> usize {
    let mut seen = HashSet::new();
    let mut duplicates = 0;
    for command in commands {
        if !seen.insert(command) {
            duplicates += 1;
        }
    }
    duplicates
}
