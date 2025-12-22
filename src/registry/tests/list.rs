use super::super::list::list_commands;
use super::super::path::{data_dir, ensure_dir, registry_path};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn list_returns_empty_when_file_missing() {
    // 未作成の登録ファイルは空として扱われる。
    let _temp_home = TestHome::new();
    let commands = list_commands().expect("list should succeed");
    assert!(commands.is_empty());
}

#[test]
fn list_returns_commands_in_order() {
    // 登録済みコマンドが順序通りに返ることを確認する。
    let temp_home = TestHome::new();
    let registry_path = temp_home.registry_path();
    // 登録ファイルを書き込むため、親ディレクトリを先に用意する。
    let registry_dir = registry_path
        .parent()
        .expect("registry dir should exist");
    ensure_dir(registry_dir).expect("ensure registry dir should succeed");

    let contents = r#"
commands = ["ls", "pwd"]
"#;
    fs::write(&registry_path, contents.trim_start()).expect("write should succeed");

    let commands = list_commands().expect("list should succeed");
    assert_eq!(commands, vec!["ls".to_string(), "pwd".to_string()]);
}

struct TestHome {
    original_home: Option<String>,
    temp_dir: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

impl TestHome {
    fn new() -> Self {
        // テストが並列に動いても衝突しないようにユニークなディレクトリを作る。
        let temp_dir = unique_temp_dir();
        ensure_dir(&temp_dir).expect("ensure dir should succeed");

        // 環境変数の書き換えは全体に影響するため、排他ロックで保護する。
        let guard = env_mutex().lock().expect("env mutex should lock");
        let original_home = std::env::var("HOME").ok();
        // テスト実行中にのみHOMEを差し替えるため、安全性をコメントで明示してunsafeで包む。
        unsafe {
            std::env::set_var("HOME", &temp_dir);
        }

        Self {
            original_home,
            temp_dir,
            _guard: guard,
        }
    }

    fn registry_path(&self) -> PathBuf {
        let dir = data_dir().expect("data dir should exist");
        registry_path(&dir)
    }

}

impl Drop for TestHome {
    fn drop(&mut self) {
        // テスト終了時に環境変数と一時ディレクトリを確実に戻す。
        unsafe {
            if let Some(original) = &self.original_home {
                std::env::set_var("HOME", original);
            } else {
                std::env::remove_var("HOME");
            }
        }
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}

/// 環境変数操作を直列化するためのミューテックス。
///
/// 並列テスト中の競合を避け、unsafe操作の前提を満たす。
fn env_mutex() -> &'static Mutex<()> {
    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_MUTEX.get_or_init(|| Mutex::new(()))
}

/// テスト用の一時ディレクトリを生成する。
///
/// PIDと現在時刻を組み合わせ、衝突しにくい名前を作る。
fn unique_temp_dir() -> PathBuf {
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("gclip-test-home-{pid}-{nanos}"))
}
