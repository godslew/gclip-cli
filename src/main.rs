use clap::Parser;
use gclip_cli::add;
use gclip_cli::cli;
use gclip_cli::list;
use gclip_cli::search;
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
    if cli.init {
        search::print_init_script();
        return Ok(());
    }

    if cli.zsh_widget {
        search::print_zsh_widget();
        return Ok(());
    }

    if cli.suggest {
        return suggest::run();
    }

    if cli.list {
        return list::run();
    }

    if let Some(command) = cli.add {
        return add::run(&command);
    }

    if let Some(query) = cli.query {
        return search::run(&query);
    }

    Ok(())
}
