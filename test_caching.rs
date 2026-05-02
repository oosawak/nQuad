// キャッシング機能テスト（独立実行用）

use nantaraquad::engine::cache::{CacheKey, CompositeCache, CacheStats};
use nantaraquad::resource::{ColorMode, SpriteData};

fn create_test_sprite() -> SpriteData {
    SpriteData::new(32, 32, ColorMode::FullColor)
}

fn main() {
    println!("=== キャッシング機能テスト ===\n");

    // Test 1: 基本操作
    {
        println!("Test 1: キャッシュの基本動作");
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        let data = create_test_sprite();
        
        cache.insert(key.clone(), data.clone());
        assert_eq!(cache.len(), 1);
        
        let cached = cache.get(&key);
        assert!(cached.is_some());
        println!("✓ insert() と get() が機能");
        
        let missing = cache.get(&CacheKey {
            asset_id: 2,
            frame_index: 0,
        });
        assert!(missing.is_none());
        println!("✓ 存在しないキーで None を返す\n");
    }

    // Test 2: ヒット・ミス
    {
        println!("Test 2: キャッシュヒット・ミス");
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        
        let stats_before = cache.get_stats();
        assert_eq!(stats_before.misses, 0);
        
        let _ = cache.get(&key);
        let stats = cache.get_stats();
        assert_eq!(stats.misses, 1);
        println!("✓ 初回アクセスでミスをカウント");
        
        cache.insert(key.clone(), create_test_sprite());
        let _ = cache.get(&key);
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        println!("✓ 2回目アクセスでヒットをカウント\n");
    }

    // Test 3: LRU 逐出
    {
        println!("Test 3: LRU 逐出");
        let mut cache = CompositeCache::new(3);
        
        for i in 0..3 {
            cache.insert(
                CacheKey {
                    asset_id: i,
                    frame_index: 0,
                },
                create_test_sprite(),
            );
        }
        assert_eq!(cache.len(), 3);
        
        cache.insert(
            CacheKey {
                asset_id: 3,
                frame_index: 0,
            },
            create_test_sprite(),
        );
        assert_eq!(cache.len(), 3);
        
        let stats = cache.get_stats();
        assert_eq!(stats.evictions, 1);
        println!("✓ max_entries 超過で最も古いエントリを削除");
        
        let evicted = cache.get(&CacheKey {
            asset_id: 0,
            frame_index: 0,
        });
        assert!(evicted.is_none());
        println!("✓ 削除されたエントリにはアクセスできない\n");
    }

    // Test 4: クリア
    {
        println!("Test 4: キャッシュクリア");
        let mut cache = CompositeCache::new(10);
        
        for i in 0..5 {
            cache.insert(
                CacheKey {
                    asset_id: i,
                    frame_index: 0,
                },
                create_test_sprite(),
            );
        }
        assert_eq!(cache.len(), 5);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        println!("✓ clear() で全エントリを削除\n");
    }

    // Test 5: 複数フレーム
    {
        println!("Test 5: 複数フレームのキャッシング");
        let mut cache = CompositeCache::new(10);
        
        for frame in 0..5 {
            cache.insert(
                CacheKey {
                    asset_id: 1,
                    frame_index: frame,
                },
                create_test_sprite(),
            );
        }
        assert_eq!(cache.len(), 5);
        
        for frame in 0..5 {
            let key = CacheKey {
                asset_id: 1,
                frame_index: frame,
            };
            assert!(cache.get(&key).is_some());
        }
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 5);
        println!("✓ 異なるフレームを個別にキャッシュ\n");
    }

    // Test 6: 統計リセット
    {
        println!("Test 6: キャッシュ統計");
        let mut cache = CompositeCache::new(10);
        let key = CacheKey {
            asset_id: 1,
            frame_index: 0,
        };
        
        let _ = cache.get(&key);
        let stats = cache.get_stats();
        assert_eq!(stats.misses, 1);
        
        cache.reset_stats();
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        println!("✓ 統計をリセット可能\n");
    }

    println!("=== すべてのテストに成功 ===");
}
