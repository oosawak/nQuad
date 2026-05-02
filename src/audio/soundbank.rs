use std::collections::HashMap;

/// サウンドバンク（プリセット）
pub struct SoundBank {
    bgm_tracks: HashMap<&'static str, &'static [u8]>,
    sfx_sounds: HashMap<&'static str, &'static [u8]>,
}

impl SoundBank {
    /// 新しい SoundBank を作成
    pub fn new() -> Self {
        SoundBank {
            bgm_tracks: HashMap::new(),
            sfx_sounds: HashMap::new(),
        }
    }

    /// プリセット効果音の登録
    pub fn register_presets(&mut self) {
        // Jump
        self.sfx_sounds.insert("jump", &[]);
        // Hit
        self.sfx_sounds.insert("hit", &[]);
        // Coin
        self.sfx_sounds.insert("coin", &[]);
        // Clear
        self.sfx_sounds.insert("clear", &[]);
        // GameOver
        self.sfx_sounds.insert("game_over", &[]);
    }

    /// サウンドが登録されているか確認
    pub fn has_sound(&self, name: &str) -> bool {
        self.bgm_tracks.contains_key(name) || self.sfx_sounds.contains_key(name)
    }

    /// BGM データを取得
    pub fn get_bgm(&self, name: &str) -> Option<&[u8]> {
        self.bgm_tracks.get(name).copied()
    }

    /// 効果音データを取得
    pub fn get_sfx(&self, name: &str) -> Option<&[u8]> {
        self.sfx_sounds.get(name).copied()
    }

    /// BGM を登録
    pub fn register_bgm(&mut self, name: &'static str, data: &'static [u8]) {
        self.bgm_tracks.insert(name, data);
    }

    /// 効果音を登録
    pub fn register_sfx(&mut self, name: &'static str, data: &'static [u8]) {
        self.sfx_sounds.insert(name, data);
    }
}

impl Default for SoundBank {
    fn default() -> Self {
        Self::new()
    }
}
