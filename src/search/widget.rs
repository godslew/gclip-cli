/// zsh用の挿入ウィジェットを標準出力へ出力する。
///
/// 出力されたスクリプトを `.zshrc` で読み込むことで、
/// Ctrl+gと`gclip`の両方で挿入できるようにする。
pub(super) fn print_zsh_widget() {
    let script = build_zsh_script(false);
    print!("{script}");
}

/// セットアップ用のスクリプトを標準出力へ出力する。
///
/// `.zshrc` に評価させることで、ウィジェットとキー割り当てを有効化する。
pub(super) fn print_init_script() {
    let script = build_zsh_script(true);
    print!("{script}");
}

/// 共通のzshスクリプトを生成する。
///
/// `bindkey` の有無だけを切り替え、処理本体は共通化する。
fn build_zsh_script(include_bindkey: bool) -> String {
    let mut script = String::from(ZSH_SCRIPT_BASE);
    if include_bindkey {
        script.push_str("bindkey '^g' gclip_insert\n");
    }
    script
}

const ZSH_SCRIPT_BASE: &str = r#"# gclip zsh integration
# 選択画面は一時的に別画面へ出し、確定後は元のプロンプトに戻す。
_gclip_use_alt_screen=0
_gclip_use_tput=0

# 画面切替の開始処理を共通化する。
_gclip_begin_ui() {
  _gclip_use_alt_screen=0
  _gclip_use_tput=0
  if command -v tput >/dev/null 2>&1; then
    tput smcup
    _gclip_use_alt_screen=1
    _gclip_use_tput=1
  else
    printf '\033[?1049h' > /dev/tty
    _gclip_use_alt_screen=1
  fi
}

# 画面切替の終了処理を共通化する。
_gclip_end_ui() {
  if (( _gclip_use_alt_screen )); then
    if (( _gclip_use_tput )); then
      tput rmcup
    else
      printf '\033[?1049l' > /dev/tty
    fi
  fi
}

# gclip本体を呼び出し、選択されたコマンドをREPLYへ格納する。
_gclip_pick_command() {
  local query="$1"
  local cmd

  _gclip_begin_ui
  if [[ -z "$query" ]]; then
    cmd="$(command gclip)" || { _gclip_end_ui; return 1; }
  else
    cmd="$(command gclip "$query")" || { _gclip_end_ui; return 1; }
  fi
  _gclip_end_ui

  if [[ -z "$cmd" ]]; then
    return 1
  fi
  REPLY="$cmd"
  return 0
}

# Ctrl+gから呼び出す挿入ウィジェット。
gclip_insert() {
  local original_lbuffer="$LBUFFER"
  local original_rbuffer="$RBUFFER"
  local query="$LBUFFER"
  local cmd

  if _gclip_pick_command "$query"; then
    cmd="$REPLY"
  else
    LBUFFER="$original_lbuffer"
    RBUFFER="$original_rbuffer"
    zle -R
    return
  fi

  if [[ -n "$query" && "$original_lbuffer" == *"$query" ]]; then
    LBUFFER="${original_lbuffer%$query}$cmd"
  else
    LBUFFER="${original_lbuffer}${cmd}"
  fi
  RBUFFER="$original_rbuffer"
  zle -R
}
zle -N gclip_insert

# gclipコマンドとして呼び出した場合も挿入できるようにする。
gclip() {
  local cmd

  if [[ ! -o interactive ]]; then
    command gclip "$@"
    return
  fi

  if (( $# == 0 )); then
    if _gclip_pick_command ""; then
      print -z -- "$REPLY"
    fi
    return
  fi

  if [[ "$1" == "--" ]]; then
    shift
    if _gclip_pick_command "$*"; then
      print -z -- "$REPLY"
    fi
    return
  fi

  if [[ "$1" == -* ]]; then
    command gclip "$@"
    return
  fi

  if _gclip_pick_command "$*"; then
    print -z -- "$REPLY"
  fi
}
"#;
