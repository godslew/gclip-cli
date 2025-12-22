use crate::registry::Registry;

/// 登録済みコマンドを一覧表示する。
///
/// `gclip --list` で一覧を出力する。
pub fn run() -> Result<(), String> {
    let commands = Registry::list_commands()?;
    print_commands(&commands);
    Ok(())
}

/// 登録済みコマンドを標準出力へ表示する。
///
/// 件数が0の場合も明示的に表示する。
fn print_commands(commands: &[String]) {
    if commands.is_empty() {
        println!("No registered commands.");
        return;
    }

    for (index, command) in commands.iter().enumerate() {
        println!("{:>2}. {}", index + 1, command);
    }
}
