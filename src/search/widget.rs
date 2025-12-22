/// zsh用の挿入ウィジェットを標準出力へ出力する。
///
/// 出力されたスクリプトを `.zshrc` で読み込むことで、
/// 選択したコマンドをプロンプトに挿入できる。
pub(super) fn print_zsh_widget() {
    println!(
        r#"# gclip zsh widget
gclip_insert() {{
  local query="$LBUFFER"
  local cmd

  if [[ -z "$query" ]]; then
    vared -p "gclip query: " query
  fi
  [[ -z "$query" ]] && return

  cmd="$(command gclip "$query")" || return
  [[ -n "$cmd" ]] && LBUFFER+="$cmd"
  zle -R
}}
zle -N gclip_insert
"#
    );
}

/// セットアップ用のスクリプトを標準出力へ出力する。
///
/// `.zshrc` に評価させることで、ウィジェットとキー割り当てを有効化する。
pub(super) fn print_init_script() {
    println!(
        r#"# gclip init
gclip_insert() {{
  local query="$LBUFFER"
  local cmd

  if [[ -z "$query" ]]; then
    vared -p "gclip query: " query
  fi
  [[ -z "$query" ]] && return

  cmd="$(command gclip "$query")" || return
  [[ -n "$cmd" ]] && LBUFFER+="$cmd"
  zle -R
}}
zle -N gclip_insert
bindkey '^g' gclip_insert
"#
    );
}
