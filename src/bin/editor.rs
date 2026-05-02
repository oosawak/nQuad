use nantaraquad::api::framework::{GameApp, GameRunner};
use nantaraquad::editor::document::EditorDocument;
use nantaraquad::resource::asset::SpriteId;
use nantaraquad::api::camera::Camera;
use nantaraquad::resource::data::ColorMode;

/// Sprite Editor Application
/// 
/// Features:
/// - Layer management UI
/// - Animation timeline
/// - Pixel paint tools
/// - File save/load
pub struct SpriteEditor {
    // Document state
    document: Option<EditorDocument>,
    
    // UI state
    selected_layer: Option<usize>,
    selected_frame: u32,
    current_tool: EditorTool,
    zoom_level: f32,
    
    // Canvas state
    camera: Camera,
    show_grid: bool,
    grid_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorTool {
    Pencil,
    Eraser,
    Fill,
    ColorPicker,
}

impl SpriteEditor {
    pub fn new() -> Self {
        Self {
            document: None,
            selected_layer: None,
            selected_frame: 0,
            current_tool: EditorTool::Pencil,
            zoom_level: 4.0,
            camera: Camera::new(512, 512),
            show_grid: true,
            grid_size: 1,
        }
    }
    
    pub fn new_document(&mut self, width: u32, height: u32) {
        self.document = Some(EditorDocument::new(
            SpriteId::new(),
            width,
            height,
            ColorMode::FullColor,
        ));
        self.selected_layer = Some(0);
        self.selected_frame = 0;
    }
    
    pub fn open_document(&mut self, path: &str) -> Result<(), String> {
        // TODO: Load from file
        Err("Not implemented yet".to_string())
    }
    
    pub fn save_document(&self) -> Result<(), String> {
        // TODO: Save to file
        Err("Not implemented yet".to_string())
    }
}

impl GameApp for SpriteEditor {
    fn update(&mut self, _delta_ms: f32) {
        // Update logic here
    }
    
    fn draw(&mut self) {
        // Draw canvas
        // Draw UI (layers, timeline, tools)
    }
}

fn main() {
    let editor = SpriteEditor::new();
    let mut runner = GameRunner::new(editor, "Sprite Editor", 1024, 768, 60);
    runner.run();
}
