use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;

/// 単一選択の入力を促し、選択されたインデックスを返す。
///
/// - 空入力はキャンセル扱いとして `None` を返す。
/// - 不正な入力は再入力を促す。
pub(crate) fn prompt_single_selection(max: usize, label: &str) -> Result<Option<usize>, String> {
    loop {
        print_prompt(label, max)?;
        let input = read_input_line()?;
        match parse_selection_input(&input, max) {
            Ok(selection) => return Ok(selection),
            Err(err) => {
                eprintln!("Invalid selection: {err}");
            }
        }
    }
}

/// 選択プロンプトを標準エラーへ出力し、フラッシュする。
///
/// 標準出力は挿入対象のコマンド出力に使うため、混ぜないようにする。
fn print_prompt(label: &str, max: usize) -> Result<(), String> {
    eprint!("{label} (1-{max}, empty to cancel): ");
    io::stderr()
        .flush()
        .map_err(|err| format!("failed to flush stderr: {err}"))
}

/// 標準入力から1行読み取って返す。
///
/// 入力が読めない場合はエラーにする。
fn read_input_line() -> Result<String, String> {
    // zleウィジェット内から呼ぶ場合、stdinが無効になることがあるため、
    // 明示的に /dev/tty から入力を読む。
    let tty = File::open("/dev/tty").map_err(|err| format!("failed to open /dev/tty: {err}"))?;
    let _guard = enable_tty_echo(&tty);
    let mut reader = BufReader::new(tty);
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .map_err(|err| format!("failed to read input: {err}"))?;
    Ok(input)
}

/// 端末のエコー/カノニカルモードを有効にする。
///
/// zle実行中はエコーが無効化されていることがあるため、
/// 入力が見えるように一時的に状態を変更する。
fn enable_tty_echo(tty: &File) -> Option<TtyModeGuard> {
    let fd = tty.as_raw_fd();
    let mut original_termios = MaybeUninit::<libc::termios>::uninit();
    let result = unsafe { libc::tcgetattr(fd, original_termios.as_mut_ptr()) };
    if result != 0 {
        return None;
    }

    let original = unsafe { original_termios.assume_init() };
    let mut modified = original;
    modified.c_lflag |= libc::ECHO | libc::ICANON;
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &modified);
    }

    Some(TtyModeGuard { fd, original })
}

/// 端末設定を元に戻すためのガード。
///
/// スコープを抜けると元の設定に戻る。
struct TtyModeGuard {
    fd: i32,
    original: libc::termios,
}

impl Drop for TtyModeGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
        }
    }
}

/// 選択入力を解析し、インデックスを返す。
///
/// - 空入力: `None`
/// - 数値1〜max: `Some(index)`
pub(super) fn parse_selection_input(input: &str, max: usize) -> Result<Option<usize>, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let index = parse_index(trimmed)?;
    if index == 0 || index > max {
        return Err(format!("out of range: {index}"));
    }
    Ok(Some(index))
}

/// 数値の選択インデックスを解析する。
///
/// 数値以外はエラーにする。
fn parse_index(token: &str) -> Result<usize, String> {
    token
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("not a number: {token}"))
}

#[cfg(test)]
mod tests;
