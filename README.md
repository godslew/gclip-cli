# gclip-cli

gclip is a small macOS-only CLI to collect, search, and insert frequently used shell commands.
It focuses on zsh and stores data under `~/.gclip/`.

## Build

```sh
cargo build
./target/debug/gclip --help
```

## Install (Homebrew)

```sh
brew tap godslew/tap
brew install gclip-cli
```

## Setup (zsh)

```sh
gclip --init > ~/.gclip.zsh
echo 'source ~/.gclip.zsh' >> ~/.zshrc
source ~/.zshrc
```

Default key binding is `Ctrl+g`. Both `Ctrl+g` and `gclip` insert the selected command into the prompt.

## Usage

```sh
# Search registered commands and insert from the list
gclip git

# Show the most recent 10 commands and insert
gclip

# Add a command manually
gclip --add "git status"

# List registered commands
gclip --list

# Remove a command by substring search + selection
gclip --rm "git"

# Recommend from the last 100 zsh history entries and register
gclip --suggest

# Show config paths and integrity checks
gclip --doctor
```

Notes:
- For queries starting with a dash, use `gclip -- --foo`.
- In non-interactive shells, `gclip` prints the selected command to stdout.

## Data files

- `~/.gclip/registered.toml`
- `~/.gclip/recent.toml`

`gclip --suggest` reads `~/.zsh_history`.
