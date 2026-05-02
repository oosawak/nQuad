use crate::engine::Engine;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// グローバルゲームエンジン - どこからでもアクセス可能
    pub static ref ENGINE: Mutex<Engine> = Mutex::new(Engine::new());
}

/// エンジン状態をロック取得
pub fn get_engine() -> std::sync::MutexGuard<'static, Engine> {
    ENGINE.lock().unwrap()
}
