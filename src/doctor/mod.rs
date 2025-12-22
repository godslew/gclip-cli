use crate::registry::Registry;
use std::path::PathBuf;

/// 設定/保存場所と整合性を確認する。
///
/// `gclip --doctor` で実行する。
pub fn run() -> Result<(), String> {
    let report = Registry::doctor_report()?;
    print_report(&report);
    Ok(())
}

/// 診断結果を標準出力へ表示する。
fn print_report(report: &DoctorReport) {
    println!("Data dir: {}", report.data_dir.display());
    println!("Registry file: {}", report.registry_path.display());
    println!("Data dir exists: {}", yes_no(report.data_dir_exists));
    println!("Registry file exists: {}", yes_no(report.registry_exists));
    println!("Registry readable: {}", yes_no(report.registry_readable));
    println!("Registry valid: {}", yes_no(report.registry_valid));
    println!("Command count: {}", report.command_count);
    println!("Empty commands: {}", report.empty_commands);
    println!("Duplicate commands: {}", report.duplicate_commands);

    if !report.errors.is_empty() {
        println!("Errors:");
        for error in &report.errors {
            println!("- {error}");
        }
    }
}

/// 表示用に真偽値をyes/noへ変換する。
fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

/// doctorで使う診断結果の構造体。
pub struct DoctorReport {
    pub data_dir: PathBuf,
    pub registry_path: PathBuf,
    pub data_dir_exists: bool,
    pub registry_exists: bool,
    pub registry_readable: bool,
    pub registry_valid: bool,
    pub command_count: usize,
    pub empty_commands: usize,
    pub duplicate_commands: usize,
    pub errors: Vec<String>,
}
