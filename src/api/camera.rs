/// カメラ・ビューポートシステム
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub scale: f32, // ズームレベル（1.0 = 1:1）
}

impl Camera {
    /// 新規カメラを作成
    pub fn new(width: u32, height: u32) -> Self {
        Camera {
            x: 0.0,
            y: 0.0,
            width,
            height,
            scale: 1.0,
        }
    }

    /// world 座標を screen 座標に変換
    pub fn world_to_screen(&self, wx: f32, wy: f32) -> (i32, i32) {
        let sx = ((wx - self.x) * self.scale) as i32;
        let sy = ((wy - self.y) * self.scale) as i32;
        (sx, sy)
    }

    /// screen 座標を world 座標に変換
    pub fn screen_to_world(&self, sx: i32, sy: i32) -> (f32, f32) {
        let wx = self.x + (sx as f32 / self.scale);
        let wy = self.y + (sy as f32 / self.scale);
        (wx, wy)
    }

    /// target に camera を移動（smooth follow）
    pub fn follow(&mut self, target_x: f32, target_y: f32, speed: f32) {
        let center_x = self.x + (self.width as f32) / 2.0 / self.scale;
        let center_y = self.y + (self.height as f32) / 2.0 / self.scale;

        let dx = target_x - center_x;
        let dy = target_y - center_y;

        self.x += dx * speed;
        self.y += dy * speed;
    }

    /// ズームレベルを設定
    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom > 0.0 {
            self.scale = zoom;
        }
    }

    /// ズームイン
    pub fn zoom_in(&mut self, factor: f32) {
        if factor > 0.0 {
            self.scale *= factor;
        }
    }

    /// ズームアウト
    pub fn zoom_out(&mut self, factor: f32) {
        if factor > 0.0 {
            self.scale /= factor;
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(160, 120)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(160, 120);
        assert_eq!(camera.width, 160);
        assert_eq!(camera.height, 120);
        assert_eq!(camera.x, 0.0);
        assert_eq!(camera.y, 0.0);
        assert_eq!(camera.scale, 1.0);
    }

    #[test]
    fn test_world_to_screen() {
        let camera = Camera::new(160, 120);
        let (sx, sy) = camera.world_to_screen(100.0, 100.0);
        assert_eq!(sx, 100);
        assert_eq!(sy, 100);
    }

    #[test]
    fn test_world_to_screen_with_offset() {
        let mut camera = Camera::new(160, 120);
        camera.x = 50.0;
        camera.y = 50.0;
        let (sx, sy) = camera.world_to_screen(100.0, 100.0);
        assert_eq!(sx, 50);
        assert_eq!(sy, 50);
    }

    #[test]
    fn test_screen_to_world() {
        let camera = Camera::new(160, 120);
        let (wx, wy) = camera.screen_to_world(100, 100);
        assert_eq!(wx, 100.0);
        assert_eq!(wy, 100.0);
    }

    #[test]
    fn test_follow() {
        let mut camera = Camera::new(160, 120);
        camera.follow(200.0, 200.0, 0.1);
        assert!(camera.x > 0.0);
        assert!(camera.y > 0.0);
    }

    #[test]
    fn test_zoom() {
        let mut camera = Camera::new(160, 120);
        camera.set_zoom(2.0);
        assert_eq!(camera.scale, 2.0);
    }
}
