use crate::audio::{AudioManager, SoundBank};

/// Pyxel 互換オーディオAPI
pub struct PyxelAudio {
    manager: AudioManager,
    soundbank: SoundBank,
}

impl PyxelAudio {
    /// 新しい PyxelAudio を作成
    pub fn new() -> Self {
        let mut soundbank = SoundBank::new();
        soundbank.register_presets();
        
        PyxelAudio {
            manager: AudioManager::new(),
            soundbank,
        }
    }

    /// 効果音を再生（Pyxel互換）
    /// sfx(id, [note_list], duration, tempo, [volume], loop)
    pub fn sfx(
        &self,
        id: u32,
        _notes: Option<Vec<u32>>,
        _duration: u32,
        _tempo: u32,
        _volume: Option<f32>,
        _looped: bool,
    ) -> Result<(), String> {
        let sfx_names = [
            "jump",
            "hit",
            "coin",
            "clear",
            "game_over",
        ];
        
        if (id as usize) < sfx_names.len() {
            let name = sfx_names[id as usize];
            self.manager.play_sfx(name)
        } else {
            Err(format!("SFX id {} not found", id))
        }
    }

    /// BGM を再生（Pyxel互換）
    /// music(section, [fine], fade_out)
    pub fn music(
        &mut self,
        section: u32,
        _fine: Option<u32>,
        _fade_out: Option<u32>,
    ) -> Result<(), String> {
        let bgm_names = [
            "forest",
            "town",
            "dungeon",
            "boss",
        ];
        
        if (section as usize) < bgm_names.len() {
            let name = bgm_names[section as usize];
            self.manager.play_bgm(name)
        } else {
            Err(format!("BGM section {} not found", section))
        }
    }

    /// AudioManager への参照を取得
    pub fn get_manager(&self) -> &AudioManager {
        &self.manager
    }

    /// AudioManager への可変参照を取得
    pub fn get_manager_mut(&mut self) -> &mut AudioManager {
        &mut self.manager
    }

    /// SoundBank への参照を取得
    pub fn get_soundbank(&self) -> &SoundBank {
        &self.soundbank
    }

    /// SoundBank への可変参照を取得
    pub fn get_soundbank_mut(&mut self) -> &mut SoundBank {
        &mut self.soundbank
    }
}

impl Default for PyxelAudio {
    fn default() -> Self {
        Self::new()
    }
}
