/// アイソメトリック投影システム
///
/// 3D座標を2Dスクリーン座標に変換
/// マリオ64風の斜め視点を実現

use super::vec3::Vec3;
use std::f32::consts::PI;

/// アイソメトリック投影計算
pub struct IsometricProjector {
    /// スケール（奥行きによる縮小率）
    pub scale_factor: f32,
    /// アイソメトリック角度（通常 45°）
    pub angle_x: f32,
    pub angle_y: f32,
}

impl IsometricProjector {
    /// 新規投影器を作成
    pub fn new() -> Self {
        IsometricProjector {
            scale_factor: 1.0,
            angle_x: -0.463647609f32,  // -26.565°（アイソメトリック標準角）
            angle_y: 0.785398163f32,   // 45°
        }
    }

    /// 3D座標を2D座標に投影
    ///
    /// 返り値は (screen_x, screen_y, z_for_depth_sorting)
    pub fn project(&self, pos: &Vec3) -> (i32, i32, f32) {
        // 回転適用
        let rotated = pos.rotate_x(self.angle_x).rotate_y(self.angle_y);

        // スクリーン座標に変換
        // アイソメトリック投影の標準的な公式
        let screen_x = rotated.x - rotated.y;
        let screen_y = (rotated.x + rotated.y) * 0.5 - rotated.z;

        (screen_x as i32, screen_y as i32, rotated.z)
    }

    /// 複数オブジェクトを投影し、深度順でソート
    pub fn project_and_sort<T: Clone>(&self, objects: &[(Vec3, T)]) -> Vec<(i32, i32, T)> {
        let mut projected: Vec<(i32, i32, f32, T)> = objects
            .iter()
            .map(|(pos, obj)| {
                let (sx, sy, z) = self.project(pos);
                (sx, sy, z, obj.clone())
            })
            .collect();

        // Z値でソート（小さい順 = 手前）
        projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        projected.into_iter().map(|(sx, sy, _, obj)| (sx, sy, obj)).collect()
    }

    /// カメラ角度を設定（0°, 90°, 180°, 270°）
    pub fn set_camera_angle(&mut self, angle_degrees: f32) {
        let angle_rad = angle_degrees * PI / 180.0;
        self.angle_y = angle_rad + 0.785398163f32;  // 45° + カメラ回転
    }
}

impl Default for IsometricProjector {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D対応カメラ
pub struct IsoCamera {
    pub position: Vec3,
    pub rotation: f32,  // ラジアン
    pub zoom: f32,
}

impl IsoCamera {
    /// 新規カメラを作成
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        IsoCamera {
            position: Vec3::new(x, y, z),
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    /// プレイヤーを追従（スムーズ）
    pub fn follow(&mut self, target: &Vec3, speed: f32) {
        let direction = *target - self.position;
        let distance = direction.magnitude();

        if distance > 0.1 {
            let normalized = direction.normalize();
            self.position = self.position + normalized * (distance * speed);
        }
    }

    /// カメラを左回転
    pub fn rotate_left(&mut self, angle: f32) {
        self.rotation += angle;
    }

    /// カメラを右回転
    pub fn rotate_right(&mut self, angle: f32) {
        self.rotation -= angle;
    }

    /// ズーム設定
    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom > 0.1 {
            self.zoom = zoom;
        }
    }

    /// 視点のローカル座標をワールド座標に変換
    pub fn local_to_world(&self, local: &Vec3) -> Vec3 {
        local.rotate_z(self.rotation) + self.position
    }

    /// ワールド座標を視点ローカル座標に変換
    pub fn world_to_local(&self, world: &Vec3) -> Vec3 {
        (*world - self.position).rotate_z(-self.rotation)
    }
}

impl Default for IsoCamera {
    fn default() -> Self {
        Self::new(0.0, 0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projector_creation() {
        let projector = IsometricProjector::new();
        assert!(projector.scale_factor > 0.0);
    }

    #[test]
    fn test_projection() {
        let projector = IsometricProjector::new();
        let pos = Vec3::new(50.0, 50.0, 0.0);
        let (sx, sy, z) = projector.project(&pos);
        assert!(sx >= -1000 && sx <= 1000);
        assert!(sy >= -1000 && sy <= 1000);
    }

    #[test]
    fn test_camera_creation() {
        let camera = IsoCamera::new(0.0, 0.0, 100.0);
        assert_eq!(camera.position.z, 100.0);
    }

    #[test]
    fn test_camera_rotation() {
        let mut camera = IsoCamera::new(0.0, 0.0, 100.0);
        let initial_rotation = camera.rotation;
        camera.rotate_left(0.1);
        assert_ne!(camera.rotation, initial_rotation);
    }
}
