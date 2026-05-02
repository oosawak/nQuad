//! コンポジットキャッシング層（Phase 7 Task 2）
//!
//! 複数エンティティが同じフレームを繰り返し render() する際の
//! セル合成計算をキャッシュして、パフォーマンス改善を実現する。

use crate::resource::SpriteData;
use std::collections::HashMap;
use std::time::Instant;

/// キャッシュキー：アセットID + フレームインデックス
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// SpriteAsset の id
    pub asset_id: usize,
    /// フレーム番号
    pub frame_index: u32,
}

/// キャッシュされたコンポジット
#[derive(Clone, Debug)]
pub struct CachedComposite {
    /// キャッシュされたスプライトデータ
    pub data: SpriteData,
    /// キャッシュ作成時刻
    pub created_at: Instant,
}

/// キャッシュ統計情報
#[derive(Clone, Debug, Default)]
pub struct CacheStats {
    /// キャッシュヒット数
    pub hits: u64,
    /// キャッシュミス数
    pub misses: u64,
    /// キャッシュエビクション数
    pub evictions: u64,
}

/// コンポジットキャッシュ
///
/// SpriteAnimator が複数回 render() する際、セル合成計算の
/// 結果をキャッシュする。LRU アルゴリズムで、最も古いエントリが
/// max_entries に達すると削除される。
#[derive(Clone, Debug)]
pub struct CompositeCache {
    /// キャッシュデータ
    cache: HashMap<CacheKey, CachedComposite>,
    /// 最大キャッシュエントリ数
    max_entries: usize,
    /// キャッシュ統計
    stats: CacheStats,
}

impl CompositeCache {
    /// 新規キャッシュを作成
    ///
    /// # 引数
    /// - `max_entries`: 最大キャッシュエントリ数
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            stats: CacheStats::default(),
        }
    }

    /// キャッシュからデータを取得
    ///
    /// キャッシュヒット時は Some(data) を返し、ヒット数をインクリメント。
    /// キャッシュミス時は None を返し、ミス数をインクリメント。
    ///
    /// # 引数
    /// - `key`: キャッシュキー
    ///
    /// # 戻り値
    /// キャッシュされたデータ、またはキャッシュミス時は None
    pub fn get(&mut self, key: &CacheKey) -> Option<SpriteData> {
        if let Some(cached) = self.cache.get(key) {
            self.stats.hits += 1;
            Some(cached.data.clone())
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// キャッシュにデータを挿入
    ///
    /// キャッシュサイズが max_entries に達している場合、
    /// 最も古いエントリを削除（LRU 逐出）してから挿入する。
    ///
    /// # 引数
    /// - `key`: キャッシュキー
    /// - `data`: スプライトデータ
    pub fn insert(&mut self, key: CacheKey, data: SpriteData) {
        // 最大キャッシュサイズに達している場合、最も古いエントリを削除
        if self.cache.len() >= self.max_entries {
            if let Some(oldest_key) = self
                .cache
                .iter()
                .min_by_key(|(_, v)| v.created_at)
                .map(|(k, _)| k.clone())
            {
                self.cache.remove(&oldest_key);
                self.stats.evictions += 1;
            }
        }

        self.cache.insert(
            key,
            CachedComposite {
                data,
                created_at: Instant::now(),
            },
        );
    }

    /// キャッシュをすべてクリア
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// キャッシュから指定キーを削除
    ///
    /// # 引数
    /// - `key`: キャッシュキー
    ///
    /// # 戻り値
    /// 削除されたデータ、またはキー不在時は None
    pub fn remove(&mut self, key: &CacheKey) -> Option<SpriteData> {
        self.cache.remove(key).map(|cached| cached.data)
    }

    /// キャッシュ統計を取得
    pub fn get_stats(&self) -> CacheStats {
        self.stats.clone()
    }

    /// キャッシュエントリ数を取得
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// キャッシュが空かどうか
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// 最大キャッシュサイズを取得
    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    /// キャッシュ統計をリセット
    pub fn reset_stats(&mut self) {
        self.stats = CacheStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ColorMode;

    /// ヘルパー関数：テスト用 SpriteData を作成
    fn create_test_sprite_data(width: u32, height: u32) -> SpriteData {
        SpriteData::new(width, height, ColorMode::FullColor)
    }

    /// Test 1: キャッシュの基本動作
    #[test]
    fn test_cache_basic_operations() {
        let mut cache = CompositeCache::new(10);

        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        let data = create_test_sprite_data(32, 32);

        // insert() で保存
        cache.insert(key.clone(), data.clone());
        assert_eq!(cache.len(), 1);

        // get() で取得
        let cached = cache.get(&key);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().width, 32);

        // 存在しないキーは None を返す
        let missing_key = CacheKey {
            asset_id: 2,
            frame_index: 0,
        };
        let missing = cache.get(&missing_key);
        assert!(missing.is_none());
    }

    /// Test 2: キャッシュヒット・ミス
    #[test]
    fn test_cache_hit_miss() {
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        let data = create_test_sprite_data(32, 32);

        // 初回：キャッシュミス
        assert_eq!(cache.stats.misses, 0);
        let _ = cache.get(&key);
        assert_eq!(cache.stats.misses, 1);
        assert_eq!(cache.stats.hits, 0);

        // キャッシュに挿入
        cache.insert(key.clone(), data);

        // 2回目：キャッシュヒット
        let _ = cache.get(&key);
        assert_eq!(cache.stats.hits, 1);
        assert_eq!(cache.stats.misses, 1);
    }

    /// Test 3: LRU 逐出
    #[test]
    fn test_lru_eviction() {
        let mut cache = CompositeCache::new(3);
        let data = create_test_sprite_data(32, 32);

        // 3つのエントリを追加
        for i in 0..3 {
            let key = CacheKey {
                asset_id: i,
                frame_index: 0,
            };
            cache.insert(key, data.clone());
        }
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.stats.evictions, 0);

        // 4番目を追加：最も古いエントリが削除される
        let key4 = CacheKey {
            asset_id: 3,
            frame_index: 0,
        };
        cache.insert(key4.clone(), data.clone());
        assert_eq!(cache.len(), 3); // max_entries を超えない
        assert_eq!(cache.stats.evictions, 1);

        // 最初のエントリが削除されている
        let key0 = CacheKey {
            asset_id: 0,
            frame_index: 0,
        };
        assert!(cache.get(&key0).is_none());
    }

    /// Test 4: キャッシュクリア
    #[test]
    fn test_cache_clear() {
        let mut cache = CompositeCache::new(10);
        let data = create_test_sprite_data(32, 32);

        // エントリを追加
        for i in 0..5 {
            let key = CacheKey {
                asset_id: i,
                frame_index: 0,
            };
            cache.insert(key, data.clone());
        }
        assert_eq!(cache.len(), 5);

        // クリア
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    /// Test 5: キャッシュ削除
    #[test]
    fn test_cache_remove() {
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        let data = create_test_sprite_data(32, 32);

        cache.insert(key.clone(), data.clone());
        assert_eq!(cache.len(), 1);

        // 削除
        let removed = cache.remove(&key);
        assert!(removed.is_some());
        assert_eq!(cache.len(), 0);

        // 再度削除を試みる
        let removed_again = cache.remove(&key);
        assert!(removed_again.is_none());
    }

    /// Test 6: キャッシュ統計
    #[test]
    fn test_cache_statistics() {
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        let data = create_test_sprite_data(32, 32);

        // 初期状態
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.evictions, 0);

        // ミス
        let _ = cache.get(&key);
        let stats = cache.get_stats();
        assert_eq!(stats.misses, 1);

        // 挿入とヒット
        cache.insert(key.clone(), data);
        let _ = cache.get(&key);
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);

        // 統計をリセット
        cache.reset_stats();
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    /// Test 7: 複数フレームのキャッシング
    #[test]
    fn test_multiple_frames_caching() {
        let mut cache = CompositeCache::new(10);
        let data = create_test_sprite_data(32, 32);

        // 同じアセット、異なるフレームをキャッシュ
        for frame in 0..5 {
            let key = CacheKey {
                asset_id: 1,
                frame_index: frame,
            };
            cache.insert(key, data.clone());
        }
        assert_eq!(cache.len(), 5);

        // すべてのフレームにアクセス
        for frame in 0..5 {
            let key = CacheKey {
                asset_id: 1,
                frame_index: frame,
            };
            assert!(cache.get(&key).is_some());
        }
        assert_eq!(cache.stats.hits, 5);
        assert_eq!(cache.stats.misses, 0);
    }

    /// Test 8: キャッシュサイズ制限
    #[test]
    fn test_cache_size_limit() {
        let mut cache = CompositeCache::new(5);
        let data = create_test_sprite_data(32, 32);

        // max_entries を超えるエントリを追加
        for i in 0..10 {
            let key = CacheKey {
                asset_id: i,
                frame_index: 0,
            };
            cache.insert(key, data.clone());
        }

        // キャッシュサイズは max_entries を超えない
        assert_eq!(cache.len(), 5);
        // エビクションが発生している
        assert_eq!(cache.stats.evictions, 5);
    }
}
