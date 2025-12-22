use super::super::path::{data_dir, ensure_dir, registry_path};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// レジストリ周りのテストで使うHOME差し替え用のガード。
///
/// 環境変数はプロセス全体で共有されるため、
/// 排他制御と一時ディレクトリの後始末をここでまとめる。
pub(crate) struct TestHome {
    original_home: Option<String>,
    temp_dir: PathBuf,
    _guard: MutexGuard<'static, ()>,
}

impl TestHome {
    pub(crate) fn new() -> Self {
        // テストが並列に動いても衝突しないようにユニークなディレクトリを作る。
        let temp_dir = unique_temp_dir();
        ensure_dir(&temp_dir).expect("ensure dir should succeed");

        // 環境変数の書き換えは全体に影響するため、排他ロックで保護する。
        let guard = env_mutex();
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

    pub(crate) fn registry_path(&self) -> PathBuf {
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
/// 既存の失敗でポイズンしていても、後始末は継続できるようにする。
fn env_mutex() -> MutexGuard<'static, ()> {
    static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
    ENV_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|err| err.into_inner())
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
