//! 合成キャッシング（Phase 6.5）
//!
//! レイヤー合成結果をキャッシュし、dirty flag で無効化を管理。
//! 毎フレーム合成を避けてパフォーマンスを向上。

use crate::resource::SpriteData;
use std::sync::{Arc, Mutex};

/// キャッシュの統計情報
#[derive(Clone, Debug, Default)]
pub struct CacheStats {
    /// 合成実行回数
    pub compose_count: u64,
    /// キャッシュヒット数
    pub hit_count: u64,
    /// キャッシュミス数
    pub miss_count: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
}

/// 合成キャッシュ：レイヤー合成結果を保持
#[derive(Clone, Debug)]
pub struct CompositeCache {
    /// キャッシュされた合成画像
    cached: Option<SpriteData>,
    /// キャッシュが有効か（true = 使える、false = 再計算が必要）
    is_dirty: bool,
    /// 統計情報
    stats: Arc<Mutex<CacheStats>>,
}

impl CompositeCache {
    /// 新規キャッシュを作成（空の状態）
    pub fn new() -> Self {
        Self {
            cached: None,
            is_dirty: true,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// キャッシュが有効か
    pub fn is_valid(&self) -> bool {
        !self.is_dirty && self.cached.is_some()
    }

    /// キャッシュを取得
    pub fn get(&self) -> Option<&SpriteData> {
        if self.is_valid() {
            if let Ok(mut stats) = self.stats.lock() {
                stats.hit_count += 1;
            }
            self.cached.as_ref()
        } else {
            if let Ok(mut stats) = self.stats.lock() {
                stats.miss_count += 1;
            }
            None
        }
    }

    /// キャッシュに値を設定（合成実行後に呼び出す）
    pub fn set(&mut self, sprite: SpriteData) {
        self.cached = Some(sprite);
        self.is_dirty = false;
        if let Ok(mut stats) = self.stats.lock() {
            stats.compose_count += 1;
        }
    }

    /// キャッシュを無効化（レイヤー変更時に呼び出す）
    pub fn invalidate(&mut self) {
        self.is_dirty = true;
    }

    /// キャッシュをクリア
    pub fn clear(&mut self) {
        self.cached = None;
        self.is_dirty = true;
    }

    /// 統計情報を取得
    pub fn stats(&self) -> CacheStats {
        self.stats.lock().map(|s| s.clone()).unwrap_or_default()
    }

    /// 統計情報をリセット
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            *stats = CacheStats::default();
        }
    }

    /// キャッシュサイズを推定（バイト）
    pub fn estimated_size(&self) -> usize {
        self.cached
            .as_ref()
            .map(|s| {
                let pixels_size = s.pixels.len();
                pixels_size + 128 // メタデータ分を粗く見積もり
            })
            .unwrap_or(0)
    }
}

impl Default for CompositeCache {
    fn default() -> Self {
        Self::new()
    }
}

/// キャッシュの戦略（future: LRU など高度なキャッシング）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheStrategy {
    /// シンプル：1 つの最新合成結果のみ
    Simple,
    // 予約: LRU(usize),
    // 予約: TimeBasedExpiry(std::time::Duration),
}

impl Default for CacheStrategy {
    fn default() -> Self {
        CacheStrategy::Simple
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    #[test]
    fn test_cache_creation() {
        let cache = CompositeCache::new();
        assert!(!cache.is_valid());
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = CompositeCache::new();
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);

        cache.set(sprite.clone());
        assert!(cache.is_valid());
        assert!(cache.get().is_some());
    }

    #[test]
    fn test_cache_invalidate() {
        let mut cache = CompositeCache::new();
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);

        cache.set(sprite);
        assert!(cache.is_valid());

        cache.invalidate();
        assert!(!cache.is_valid());
        assert!(cache.get().is_none());
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = CompositeCache::new();
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);

        cache.set(sprite);
        let _ = cache.get(); // hit
        let _ = cache.get(); // hit

        cache.invalidate();
        let _ = cache.get(); // miss

        let stats = cache.stats();
        assert_eq!(stats.compose_count, 1);
        assert_eq!(stats.hit_count, 2);
        assert_eq!(stats.miss_count, 1);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut cache = CompositeCache::new();
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);

        cache.set(sprite);
        for _ in 0..10 {
            let _ = cache.get();
        }

        let stats = cache.stats();
        assert!((stats.hit_rate() - 1.0).abs() < 0.01); // 100% hit
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = CompositeCache::new();
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);

        cache.set(sprite);
        assert!(cache.is_valid());

        cache.clear();
        assert!(!cache.is_valid());
        assert!(cache.cached.is_none());
    }

    #[test]
    fn test_estimated_size() {
        let mut cache = CompositeCache::new();
        assert_eq!(cache.estimated_size(), 0);

        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        cache.set(sprite);

        let size = cache.estimated_size();
        assert!(size > 0); // 32 * 32 * 4 + overhead
    }
}
