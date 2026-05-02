use nantaraquad::audio::{AudioManager, SoundBank};
use nantaraquad::api::PyxelAudio;
use nantaraquad::GameEngine;

/// Test 1: AudioManager 初期化
#[test]
fn test_audio_manager_initialization() {
    let audio = AudioManager::new();
    assert_eq!(audio.get_master_volume(), 1.0);
    assert_eq!(audio.get_bgm_volume(), 1.0);
    assert_eq!(audio.get_sfx_volume(), 1.0);
}

/// Test 2: BGM 再生状態追跡
#[test]
fn test_bgm_playback_state() {
    let mut audio = AudioManager::new();
    
    // ファイルが存在しない場合はエラー
    let result = audio.play_bgm("forest");
    assert!(result.is_err());
    assert!(!audio.is_bgm_playing("forest"));
    
    // 停止時は再生状態が false
    audio.stop_bgm();
    assert!(!audio.is_bgm_playing("forest"));
}

/// Test 3: SFX 再生状態追跡
#[test]
fn test_sfx_playback() {
    let audio = AudioManager::new();
    
    // ファイルが存在しない場合はエラー
    let result = audio.play_sfx("jump");
    assert!(result.is_err());
}

/// Test 4: ボリューム制御
#[test]
fn test_volume_control() {
    let mut audio = AudioManager::new();
    
    // マスターボリュームを 0.5 に設定
    audio.set_master_volume(0.5);
    assert_eq!(audio.get_master_volume(), 0.5);
    
    // BGM ボリュームを 0.8 に設定
    audio.set_bgm_volume(0.8);
    assert_eq!(audio.get_bgm_volume(), 0.8);
    
    // 効果音ボリュームを 0.6 に設定
    audio.set_sfx_volume(0.6);
    assert_eq!(audio.get_sfx_volume(), 0.6);
}

/// Test 5: ボリュームクランプ
#[test]
fn test_volume_clamping() {
    let mut audio = AudioManager::new();
    
    // 1.5 は 1.0 にクランプされる
    audio.set_master_volume(1.5);
    assert_eq!(audio.get_master_volume(), 1.0);
    
    // -0.5 は 0.0 にクランプされる
    audio.set_master_volume(-0.5);
    assert_eq!(audio.get_master_volume(), 0.0);
    
    // 0.75 は有効な値
    audio.set_master_volume(0.75);
    assert_eq!(audio.get_master_volume(), 0.75);
}

/// Test 6: SoundBank プリセット
#[test]
fn test_soundbank_presets() {
    let mut soundbank = SoundBank::new();
    soundbank.register_presets();
    
    // プリセット効果音が登録される
    assert!(soundbank.has_sound("jump"));
    assert!(soundbank.has_sound("hit"));
    assert!(soundbank.has_sound("coin"));
    assert!(soundbank.has_sound("clear"));
    assert!(soundbank.has_sound("game_over"));
    
    // 登録されていないサウンドは見つからない
    assert!(!soundbank.has_sound("unknown"));
}

/// Test 7: SoundBank でのサウンドデータ登録
#[test]
fn test_soundbank_data_registration() {
    let mut soundbank = SoundBank::new();
    
    let test_data: &[u8] = &[0x00, 0x01, 0x02, 0x03];
    soundbank.register_sfx("test_sound", test_data);
    
    assert!(soundbank.has_sound("test_sound"));
    assert_eq!(soundbank.get_sfx("test_sound"), Some(test_data));
}

/// Test 8: PyxelAudio API 初期化
#[test]
fn test_pyxel_audio_initialization() {
    let pyxel = PyxelAudio::new();
    
    // SoundBank が初期化されている
    let soundbank = pyxel.get_soundbank();
    assert!(soundbank.has_sound("jump"));
}

/// Test 9: GameEngine オーディオ統合
#[test]
fn test_game_engine_audio_integration() {
    let engine = GameEngine::new(160, 120, 60);
    
    // engine に audio フィールドが存在する
    assert_eq!(engine.audio.get_master_volume(), 1.0);
    assert_eq!(engine.audio.get_bgm_volume(), 1.0);
    assert_eq!(engine.audio.get_sfx_volume(), 1.0);
}

/// Test 10: GameEngine with_audio ビルダーパターン
#[test]
fn test_game_engine_with_audio_builder() {
    let mut audio = AudioManager::new();
    audio.set_master_volume(0.5);
    
    let engine = GameEngine::new(160, 120, 60).with_audio(audio);
    
    assert_eq!(engine.audio.get_master_volume(), 0.5);
}

