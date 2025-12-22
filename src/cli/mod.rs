use clap::Parser;

/// CLIの引数定義をまとめるモジュール。
///
/// `clap`の自動ヘルプ生成により、
/// `gclip --help`でこの構造体の定義内容が表示される。
/// そのため、ヘルプの表現はここで集中管理する。
#[derive(Parser, Debug)]
#[command(name = "gclip", version, about = "Clipboard helper CLI")]
pub struct Cli {
    /// 直近のコマンド履歴から頻出のものを推薦し、登録まで行う。
    ///
    /// `--suggest`が指定されたときのみ履歴を読み取って処理する。
    #[arg(long, help = "Recommend frequently used commands from recent history")]
    pub suggest: bool,
}
