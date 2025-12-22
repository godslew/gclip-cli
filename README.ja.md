# gclip-cli

gclip は macOS 専用の小さな CLI で、よく使うコマンドの登録・検索・挿入を行います。
zsh 前提で、データは `~/.gclip/` に保存します。

## ビルド

```sh
cargo build
./target/debug/gclip --help
```

## zsh 連携

```sh
gclip --init > ~/.gclip.zsh
echo 'source ~/.gclip.zsh' >> ~/.zshrc
source ~/.zshrc
```

デフォルトのキー割り当ては `Ctrl+g` です。`Ctrl+g` と `gclip` の両方で、選択したコマンドをプロンプトに挿入できます。

## 使い方

```sh
# 登録済みコマンドを検索して挿入
gclip git

# 直近10件から選んで挿入
gclip

# 手動登録
gclip --add "git status"

# 登録済み一覧
gclip --list

# 部分一致検索して削除
gclip --rm "git"

# 直近100件の履歴から推薦して登録
gclip --suggest

# 設定パスと整合性の確認
gclip --doctor
```

補足:
- `-` で始まる検索は `gclip -- --foo` のように指定してください。
- 非対話シェルでは、選択結果を標準出力に出力します。

## 保存ファイル

- `~/.gclip/registered.toml`
- `~/.gclip/recent.toml`

`gclip --suggest` は `~/.zsh_history` を読み込みます。
