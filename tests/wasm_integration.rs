//! WASM Integration Tests

#[cfg(test)]
mod wasm_tests {
    use std::path::Path;

    #[test]
    fn test_cargo_toml_wasm_dependencies() {
        // Cargo.toml に wasm-bindgen が記載されているか
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(cargo_toml.contains("wasm-bindgen"), "wasm-bindgen not found in Cargo.toml");
        assert!(cargo_toml.contains("web-sys"), "web-sys not found in Cargo.toml");
        assert!(cargo_toml.contains("gloo-timers"), "gloo-timers not found in Cargo.toml");
    }

    #[test]
    fn test_wasm_profile_optimization() {
        // リリースプロファイルが WASM 最適化されているか
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(cargo_toml.contains("[profile.release]"), "Release profile not found");
        assert!(cargo_toml.contains("lto = true"), "LTO optimization not enabled");
        assert!(cargo_toml.contains("opt-level = \"z\""), "Optimization level z not set");
        assert!(cargo_toml.contains("strip = true"), "Strip not enabled");
    }

    #[test]
    fn test_wasm_entrypoints_exist() {
        // WASM エントリポイントファイルが存在するか
        let lineboy_path = Path::new("src/bin/lineboy_wasm.rs");
        let cubeboy_path = Path::new("src/bin/cubeboy_wasm.rs");

        assert!(lineboy_path.exists(), "lineboy_wasm.rs not found");
        assert!(cubeboy_path.exists(), "cubeboy_wasm.rs not found");
    }

    #[test]
    fn test_web_html_exists() {
        // HTML ローダーが存在するか
        let html_path = Path::new("web/index.html");
        assert!(html_path.exists(), "web/index.html not found");

        let index_html = include_str!("../web/index.html");
        
        // Canvas 要素が存在するか
        assert!(index_html.contains("<canvas"), "Canvas element not found");
        assert!(index_html.contains("id=\"canvas\""), "Canvas ID not found");

        // JavaScript ローダーが存在するか
        assert!(index_html.contains("startLineboy"), "startLineboy function not found");
        assert!(index_html.contains("startCubeboy"), "startCubeboy function not found");

        // WASM モジュールロードが存在するか
        assert!(index_html.contains("import"), "ES Module import not found");
    }

    #[test]
    fn test_build_script_exists() {
        // ビルドスクリプトが存在し、wasm-pack を呼び出すか
        let script_path = Path::new("scripts/build-wasm.sh");
        assert!(script_path.exists(), "build-wasm.sh not found");

        let build_script = include_str!("../scripts/build-wasm.sh");
        
        assert!(build_script.contains("wasm-pack"), "wasm-pack command not found");
        assert!(build_script.contains("lineboy"), "lineboy build not found");
        assert!(build_script.contains("cubeboy"), "cubeboy build not found");
    }

    #[test]
    fn test_serve_script_exists() {
        // サーバースクリプトが存在し、HTTP サーバーを起動するか
        let script_path = Path::new("scripts/serve-wasm.py");
        assert!(script_path.exists(), "serve-wasm.py not found");

        let serve_script = include_str!("../scripts/serve-wasm.py");
        
        assert!(serve_script.contains("http.server"), "HTTP server not found");
        assert!(serve_script.contains("8000"), "Port 8000 not found");
    }

    #[test]
    fn test_web_documentation() {
        // Web README が存在し、十分な情報を含むか
        let readme_path = Path::new("web/README.md");
        assert!(readme_path.exists(), "web/README.md not found");

        let web_readme = include_str!("../web/README.md");
        
        assert!(web_readme.contains("ビルド方法"), "Build instructions not found");
        assert!(web_readme.contains("実行方法"), "Execution instructions not found");
        assert!(web_readme.contains("対応ブラウザ"), "Browser support not documented");
        assert!(web_readme.contains("localhost"), "Server instructions not found");
    }

    #[test]
    fn test_lineboy_wasm_has_wasm_bindgen() {
        // lineboy_wasm.rs が wasm_bindgen を使用しているか
        let lineboy_path = Path::new("src/bin/lineboy_wasm.rs");
        assert!(lineboy_path.exists(), "lineboy_wasm.rs not found");
        
        let content = std::fs::read_to_string(lineboy_path)
            .expect("Failed to read lineboy_wasm.rs");
        
        assert!(content.contains("wasm_bindgen"), "wasm_bindgen not found in lineboy_wasm.rs");
        assert!(content.contains("#[wasm_bindgen(start)]"), "wasm_bindgen(start) not found in lineboy_wasm.rs");
        assert!(content.contains("cfg(target_arch = \"wasm32\")"), "WASM target cfg not found");
    }

    #[test]
    fn test_cubeboy_wasm_has_wasm_bindgen() {
        // cubeboy_wasm.rs が wasm_bindgen を使用しているか
        let cubeboy_path = Path::new("src/bin/cubeboy_wasm.rs");
        assert!(cubeboy_path.exists(), "cubeboy_wasm.rs not found");
        
        let content = std::fs::read_to_string(cubeboy_path)
            .expect("Failed to read cubeboy_wasm.rs");
        
        assert!(content.contains("wasm_bindgen"), "wasm_bindgen not found in cubeboy_wasm.rs");
        assert!(content.contains("#[wasm_bindgen(start)]"), "wasm_bindgen(start) not found in cubeboy_wasm.rs");
        assert!(content.contains("cfg(target_arch = \"wasm32\")"), "WASM target cfg not found");
    }

    #[test]
    fn test_cargo_bin_definitions() {
        // Cargo.toml にバイナリ定義があるか
        let cargo_toml = include_str!("../Cargo.toml");
        
        assert!(cargo_toml.contains("[[bin]]"), "Binary definition not found");
        assert!(cargo_toml.contains("lineboy_wasm"), "lineboy_wasm binary not defined");
        assert!(cargo_toml.contains("cubeboy_wasm"), "cubeboy_wasm binary not defined");
    }

    #[test]
    fn test_macroquad_dependency() {
        // macroquad が依存関係に含まれているか
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(cargo_toml.contains("macroquad"), "macroquad not found in dependencies");
    }

    #[test]
    fn test_wasm_target_conditional_compilation() {
        // WASM エントリポイントで条件付きコンパイルがあるか
        let lineboy_path = Path::new("src/bin/lineboy_wasm.rs");
        let lineboy_content = std::fs::read_to_string(lineboy_path)
            .expect("Failed to read lineboy_wasm.rs");
        
        let cubeboy_path = Path::new("src/bin/cubeboy_wasm.rs");
        let cubeboy_content = std::fs::read_to_string(cubeboy_path)
            .expect("Failed to read cubeboy_wasm.rs");

        // cfg(target_arch = "wasm32") を使用しているか
        assert!(lineboy_content.contains("cfg(target_arch = \"wasm32\")"), 
                "WASM target cfg not found in lineboy");
        assert!(cubeboy_content.contains("cfg(target_arch = \"wasm32\")"), 
                "WASM target cfg not found in cubeboy");

        // 非 WASM ターゲットの fallback もあるか
        assert!(lineboy_content.contains("cfg(not(target_arch = \"wasm32\"))"), 
                "Non-WASM fallback not found in lineboy");
        assert!(cubeboy_content.contains("cfg(not(target_arch = \"wasm32\"))"), 
                "Non-WASM fallback not found in cubeboy");
    }
}

