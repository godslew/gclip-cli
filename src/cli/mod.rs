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
    #[arg(short = 's', long, help = "Recommend frequently used commands from recent history")]
    pub suggest: bool,

    /// 登録済みコマンドから部分一致で検索する。
    ///
    /// 例: `gclip git` で "git" を含むコマンドを表示する。
    #[arg(
        value_name = "QUERY",
        help = "Search registered commands by substring",
        conflicts_with_all = ["add", "suggest", "zsh_widget", "init"]
    )]
    pub query: Option<String>,

    /// コマンド文字列を手動で登録する。
    ///
    /// 例: `gclip --add "git status"` で登録する。
    #[arg(
        short = 'a',
        long = "add",
        value_name = "COMMAND",
        help = "Add a command to the registry",
        conflicts_with_all = ["query", "suggest", "zsh_widget", "init"]
    )]
    pub add: Option<String>,

    /// 登録済みコマンドを一覧表示する。
    ///
    /// 例: `gclip --list` で一覧表示する。
    #[arg(
        short = 'l',
        long = "list",
        help = "List registered commands",
        conflicts_with_all = ["query", "add", "suggest", "zsh_widget", "init"]
    )]
    pub list: bool,

    /// 登録済みコマンドを削除する。
    ///
    /// 例: `gclip --rm "git status"` で削除する。
    #[arg(
        short = 'r',
        long = "rm",
        value_name = "COMMAND",
        help = "Remove a command from the registry",
        conflicts_with_all = ["query", "add", "suggest", "list", "zsh_widget", "init"]
    )]
    pub remove: Option<String>,

    /// zsh用の挿入ウィジェットを出力する。
    ///
    /// `gclip --zsh-widget` の出力を `.zshrc` から読み込む。
    #[arg(short = 'w', long, help = "Print a zsh widget script for line insertion")]
    pub zsh_widget: bool,

    /// セットアップ用のスクリプトを出力する。
    ///
    /// `gclip --init` の出力を `.zshrc` で評価する。
    #[arg(short = 'i', long, help = "Print a setup script for shell initialization")]
    pub init: bool,
}
