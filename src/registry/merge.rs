use super::RegisteredCommands;

/// 登録済みリストに新規コマンドを追加する。
///
/// 追加件数を返すことで、上位の処理が書き込み判断に利用できる。
pub(crate) fn merge_commands(
    registered: &mut RegisteredCommands,
    commands: &[String],
) -> usize {
    let mut added = 0;
    for command in commands {
        if !registered.commands.contains(command) {
            registered.commands.push(command.clone());
            added += 1;
        }
    }
    added
}
