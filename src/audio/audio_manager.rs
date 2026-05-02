use std::collections::HashMap;
use std::path::Path;

/// オーディオマネージャー
pub struct AudioManager {
    bgm_paths: HashMap<String, String>,
    sfx_paths: HashMap<String, String>,
    master_volume: f32,
    bgm_volume: f32,
    sfx_volume: f32,
    current_bgm: Option<String>,
}

impl AudioManager {
    /// 新しい AudioManager を作成
    pub fn new() -> Self {
        AudioManager {
            bgm_paths: HashMap::new(),
            sfx_paths: HashMap::new(),
            master_volume: 1.0,
            bgm_volume: 1.0,
            sfx_volume: 1.0,
            current_bgm: None,
        }
    }

    /// BGM を読み込み
    pub fn load_bgm(&mut self, name: &str, path: &str) -> Result<(), String> {
        // ファイルが存在するかチェック
        if Path::new(path).exists() {
            self.bgm_paths.insert(name.to_string(), path.to_string());
            Ok(())
        } else {
            Err(format!("Failed to load BGM '{}': file not found", path))
        }
    }

    /// 効果音を読み込み
    pub fn load_sfx(&mut self, name: &str, path: &str) -> Result<(), String> {
        // ファイルが存在するかチェック
        if Path::new(path).exists() {
            self.sfx_paths.insert(name.to_string(), path.to_string());
            Ok(())
        } else {
            Err(format!("Failed to load SFX '{}': file not found", path))
        }
    }

    /// BGM を再生
    pub fn play_bgm(&mut self, name: &str) -> Result<(), String> {
        if self.bgm_paths.contains_key(name) {
            self.current_bgm = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("BGM '{}' not found", name))
        }
    }

    /// BGM を停止
    pub fn stop_bgm(&mut self) {
        self.current_bgm = None;
    }

    /// 効果音を再生
    pub fn play_sfx(&self, name: &str) -> Result<(), String> {
        if self.sfx_paths.contains_key(name) {
            Ok(())
        } else {
            Err(format!("SFX '{}' not found", name))
        }
    }

    /// マスターボリュームを設定
    pub fn set_master_volume(&mut self, vol: f32) {
        self.master_volume = vol.clamp(0.0, 1.0);
    }

    /// BGM ボリュームを設定
    pub fn set_bgm_volume(&mut self, vol: f32) {
        self.bgm_volume = vol.clamp(0.0, 1.0);
    }

    /// 効果音ボリュームを設定
    pub fn set_sfx_volume(&mut self, vol: f32) {
        self.sfx_volume = vol.clamp(0.0, 1.0);
    }

    /// BGM が再生中か
    pub fn is_bgm_playing(&self, name: &str) -> bool {
        self.current_bgm.as_ref().map(|bgm| bgm == name).unwrap_or(false)
    }

    /// マスターボリュームを取得
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    /// BGM ボリュームを取得
    pub fn get_bgm_volume(&self) -> f32 {
        self.bgm_volume
    }

    /// 効果音ボリュームを取得
    pub fn get_sfx_volume(&self) -> f32 {
        self.sfx_volume
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
