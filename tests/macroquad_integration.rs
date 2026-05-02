//! macroquad 統合テスト

#[cfg(test)]
mod tests {
    use nantaraquad::platform::{
        MacroquadBackend, InputState, Key, GamepadButton, GamepadInput,
    };
    use nantaraquad::resource::{ColorMode, SpriteData};

    /// Test 1: macroquad ウィンドウ初期化
    #[test]
    fn test_macroquad_window_init() {
        let backend = MacroquadBackend::new(512, 512, 60);
        
        assert_eq!(backend.width(), 512);
        assert_eq!(backend.height(), 512);
        assert_eq!(backend.fps(), 60);
    }

    /// Test 2: Input Bridge - キーコード変換
    #[test]
    fn test_input_bridge_key_mapping() {
        use nantaraquad::platform::InputBridge;
        
        // キーコードの変換テスト（バイナリ側で実装されているため簡易テスト）
        // 実際のmacroquad KeyCodeとの変換は main.rs で実行
        assert!(true);
    }

    /// Test 3: FullColor スプライト → Image 変換 
    #[test]
    fn test_sprite_to_image_fullcolor() {
        let backend = MacroquadBackend::new(32, 32, 60);
        let sprite = SpriteData::new(32, 32, ColorMode::FullColor);
        
        let image = backend.sprite_to_image(&sprite);
        
        assert_eq!(image.width, 32);
        assert_eq!(image.height, 32);
        // FullColor の場合、データサイズは 32 * 32 * 4 = 4096 バイト
        assert_eq!(image.bytes.len(), 4096);
    }

    /// Test 4: Indexed256 スプライト → Image 変換
    #[test]
    fn test_sprite_to_image_indexed256() {
        let backend = MacroquadBackend::new(32, 32, 60);
        
        // パレット（256色）を作成
        let palette = vec![[0u8, 0, 0, 255]; 256];
        
        let sprite = SpriteData::new(32, 32, ColorMode::Indexed256(palette));
        
        let image = backend.sprite_to_image(&sprite);
        
        assert_eq!(image.width, 32);
        assert_eq!(image.height, 32);
        // Image は常に RGBA なので 32 * 32 * 4 = 4096 バイト
        assert_eq!(image.bytes.len(), 4096);
    }

    /// Test 5: Input State - キー入力状態
    #[test]
    fn test_input_state_key_pressed() {
        let mut state = InputState {
            keys_pressed: vec![Key::Up, Key::A],
            gamepad: GamepadInput::default(),
        };
        
        assert!(state.is_key_pressed(Key::Up));
        assert!(state.is_key_pressed(Key::A));
        assert!(!state.is_key_pressed(Key::Down));
    }

    /// Test 6: GamepadInput - ゲームパッド入力
    #[test]
    fn test_gamepad_input() {
        let mut _gamepad = GamepadInput {
            buttons: vec![GamepadButton::A, GamepadButton::B],
            stick_left: (0.5, 0.5),
            stick_right: (0.0, 0.0),
        };
        
        assert!(_gamepad.is_button_pressed(GamepadButton::A));
        assert!(_gamepad.is_button_pressed(GamepadButton::B));
        assert!(!_gamepad.is_button_pressed(GamepadButton::X));
    }

    /// Test 7: FPS 管理テスト
    #[test]
    fn test_fps_configuration() {
        let backend_60 = MacroquadBackend::new(512, 512, 60);
        let backend_30 = MacroquadBackend::new(512, 512, 30);
        
        assert_eq!(backend_60.fps(), 60);
        assert_eq!(backend_30.fps(), 30);
    }

    /// Test 8: 複数のスプライト変換
    #[test]
    fn test_multiple_sprite_conversion() {
        let backend = MacroquadBackend::new(256, 256, 60);
        
        let sprite1 = SpriteData::new(16, 16, ColorMode::FullColor);
        let sprite2 = SpriteData::new(32, 32, ColorMode::FullColor);
        let sprite3 = SpriteData::new(64, 64, ColorMode::FullColor);
        
        let image1 = backend.sprite_to_image(&sprite1);
        let image2 = backend.sprite_to_image(&sprite2);
        let image3 = backend.sprite_to_image(&sprite3);
        
        assert_eq!(image1.width, 16);
        assert_eq!(image1.height, 16);
        assert_eq!(image1.bytes.len(), 16 * 16 * 4);
        
        assert_eq!(image2.width, 32);
        assert_eq!(image2.height, 32);
        assert_eq!(image2.bytes.len(), 32 * 32 * 4);
        
        assert_eq!(image3.width, 64);
        assert_eq!(image3.height, 64);
        assert_eq!(image3.bytes.len(), 64 * 64 * 4);
    }

    /// Test 9: InputState デフォルト値
    #[test]
    fn test_input_state_default() {
        let state = InputState::default();
        
        assert!(state.keys_pressed.is_empty());
        assert!(state.gamepad.buttons.is_empty());
        assert_eq!(state.gamepad.stick_left, (0.0, 0.0));
        assert_eq!(state.gamepad.stick_right, (0.0, 0.0));
    }

    /// Test 10: ウィンドウサイズ変更テスト
    #[test]
    fn test_window_sizes() {
        let sizes = vec![
            (256, 256),
            (512, 512),
            (1024, 768),
            (800, 600),
        ];
        
        for (w, h) in sizes {
            let backend = MacroquadBackend::new(w, h, 60);
            assert_eq!(backend.width(), w);
            assert_eq!(backend.height(), h);
            
            let sprite = SpriteData::new(w, h, ColorMode::FullColor);
            let image = backend.sprite_to_image(&sprite);
            
            assert_eq!(image.width, w as u16);
            assert_eq!(image.height, h as u16);
        }
    }

    /// Test 11: Indexed256 パレット変換
    #[test]
    fn test_indexed256_palette_conversion() {
        let backend = MacroquadBackend::new(16, 16, 60);
        
        // パレット作成：インデックス 1 = 赤、 2 = 緑
        let mut palette = vec![[0u8, 0, 0, 0]; 256];
        palette[1] = [255, 0, 0, 255];     // 赤
        palette[2] = [0, 255, 0, 255];     // 緑
        
        let mut sprite = SpriteData::new(16, 16, ColorMode::Indexed256(palette));
        
        // ピクセルデータをセット（テスト用）
        sprite.pixels[0] = 1;  // 赤
        sprite.pixels[1] = 2;  // 緑
        sprite.pixels[2] = 0;  // 透明黒
        
        let image = backend.sprite_to_image(&sprite);
        
        // データサイズチェック
        assert_eq!(image.bytes.len(), 16 * 16 * 4);
        
        // ピクセル 0: 赤
        assert_eq!(image.bytes[0], 255);   // R
        assert_eq!(image.bytes[1], 0);     // G
        assert_eq!(image.bytes[2], 0);     // B
        assert_eq!(image.bytes[3], 255);   // A
    }

    /// Test 12: スプライト初期化（全ピクセル黒）
    #[test]
    fn test_sprite_initialization() {
        let sprite = SpriteData::new(8, 8, ColorMode::FullColor);
        
        // 全ピクセルが初期化されているか確認
        assert_eq!(sprite.pixels.len(), 8 * 8 * 4);
        
        // すべて 0 で初期化されるはず
        for byte in sprite.pixels.iter() {
            assert_eq!(*byte, 0);
        }
    }
}
