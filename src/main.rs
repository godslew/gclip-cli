use clap::Parser;
use gclip_cli::cli;
use gclip_cli::suggest;

fn main() {
    // mainはプロセス終了コードの責務だけを持ち、実処理はrunに委譲する。
    if let Err(err) = run() {
        eprintln!("gclip: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    // CLI引数の解析はここで行い、各機能の実装はモジュールへ分離する。
    let cli = cli::Cli::parse();
    if cli.suggest {
        return suggest::run();
    }

    Ok(())
}
