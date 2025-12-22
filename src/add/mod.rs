use crate::registry::Registry;
use std::path::Path;

/// コマンド文字列を手動で登録する機能。
///
/// `gclip --add "command"` で登録する。
pub fn run(command: &str) -> Result<(), String> {
    let (registry_path, added) = Registry::add_command(command)?;
    print_result(command, added, &registry_path);
    Ok(())
}

/// 登録結果を標準出力へ表示する。
///
/// 追加件数と登録先を明示する。
fn print_result(command: &str, added: usize, registry_path: &Path) {
    if added == 0 {
        println!("Already registered: \"{command}\"");
    } else {
        println!("Registered \"{command}\" to {}", registry_path.display());
    }
}
