//! Mario 64 - Phase 13 Prototype
//!
//! 3D座標、アイソメトリック投影、奥行きソートのテスト
//! プレイヤーが3D空間で移動し、カメラがついていく

use nantaraquad::{Vec3, IsometricProjector, IsoCamera};
use nantaraquad::api::drawing::DrawingContext;
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::api::game::GameEngine;
use std::f32::consts::PI;

struct Mario {
    position: Vec3,
    velocity: Vec3,
    size: (u32, u32),
}

impl Mario {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Mario {
            position: Vec3::new(x, y, z),
            velocity: Vec3::zero(),
            size: (4, 6),
        }
    }

    fn update(&mut self, input: &InputState, camera_angle: f32, dt: f32) {
        const MOVE_SPEED: f32 = 60.0;
        const GRAVITY: f32 = 200.0;
        const FRICTION: f32 = 0.85;

        // 入力（カメラ角度に合わせた8方向）
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if input.btn(Key::Up) || input.btn(Key::W) {
            move_y -= MOVE_SPEED;
        }
        if input.btn(Key::Down) || input.btn(Key::S) {
            move_y += MOVE_SPEED;
        }
        if input.btn(Key::Left) || input.btn(Key::A) {
            move_x -= MOVE_SPEED;
        }
        if input.btn(Key::Right) || input.btn(Key::D) {
            move_x += MOVE_SPEED;
        }

        // カメラ角度に応じて入力を回転
        let cos_a = camera_angle.cos();
        let sin_a = camera_angle.sin();
        let rotated_x = move_x * cos_a - move_y * sin_a;
        let rotated_y = move_x * sin_a + move_y * cos_a;

        self.velocity.x = rotated_x;
        self.velocity.y = rotated_y;

        // 重力
        self.velocity.z -= GRAVITY * dt;

        // 位置更新
        self.position = self.position + self.velocity * dt;

        // 地面衝突（z = 0 が地面）
        if self.position.z < 0.0 {
            self.position.z = 0.0;
            self.velocity.z = 0.0;

            // 地面では摩擦
            self.velocity.x *= FRICTION.powf(dt);
            self.velocity.y *= FRICTION.powf(dt);
        }

        // ジャンプ
        if input.btnp(Key::Space) && self.position.z < 1.0 {
            self.velocity.z = 150.0;
        }

        // ワールドバウンズ
        let bounds = 150.0;
        self.position.x = self.position.x.clamp(-bounds, bounds);
        self.position.y = self.position.y.clamp(-bounds, bounds);
    }

    fn draw(&self, drawing: &mut DrawingContext, projector: &IsometricProjector) {
        let (sx, sy, _) = projector.project(&self.position);

        // 影（z=0 での投影）
        let shadow = Vec3::new(self.position.x, self.position.y, 0.0);
        let (shadow_x, shadow_y, _) = projector.project(&shadow);
        drawing.circfill(shadow_x, shadow_y, 2, 8); // 灰色の影

        // マリオ本体
        let color = if self.position.z > 5.0 { 3 } else { 2 }; // ジャンプ中は水色
        drawing.rectfill(sx - 2, sy - 2, self.size.0, self.size.1, color);

        // 目
        drawing.pset(sx - 1, sy, 7); // 白
        drawing.pset(sx + 1, sy, 7); // 白
    }
}

struct Platform {
    position: Vec3,
    size: (f32, f32, f32),
}

impl Platform {
    fn new(x: f32, y: f32, z: f32, w: f32, h: f32, d: f32) -> Self {
        Platform {
            position: Vec3::new(x, y, z),
            size: (w, h, d),
        }
    }

    fn draw(&self, drawing: &mut DrawingContext, projector: &IsometricProjector) {
        // プラットフォームの4コーナー
        let corners = vec![
            Vec3::new(self.position.x - self.size.0 / 2.0, self.position.y - self.size.1 / 2.0, self.position.z),
            Vec3::new(self.position.x + self.size.0 / 2.0, self.position.y - self.size.1 / 2.0, self.position.z),
            Vec3::new(self.position.x + self.size.0 / 2.0, self.position.y + self.size.1 / 2.0, self.position.z),
            Vec3::new(self.position.x - self.size.0 / 2.0, self.position.y + self.size.1 / 2.0, self.position.z),
        ];

        let projected: Vec<_> = corners.iter().map(|c| projector.project(c)).collect();

        // プラットフォーム（矩形で簡略化）
        let (cx, cy, _) = projector.project(&self.position);
        drawing.rectfill(
            cx - (self.size.0 as i32 / 2),
            cy - (self.size.1 as i32 / 4),
            self.size.0 as u32,
            (self.size.1 / 2.0) as u32,
            4, // 赤
        );
    }
}

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    let mut mario = Mario::new(0.0, 0.0, 0.0);
    let mut camera = IsoCamera::new(0.0, 0.0, 100.0);
    let mut projector = IsometricProjector::new();

    // ステージのプラットフォーム
    let platforms = vec![
        Platform::new(0.0, 0.0, -5.0, 200.0, 200.0, 10.0), // メイン地面
        Platform::new(60.0, -50.0, 10.0, 60.0, 60.0, 10.0), // 浮遊台1
        Platform::new(-60.0, 50.0, 15.0, 60.0, 60.0, 10.0), // 浮遊台2
        Platform::new(0.0, 80.0, 20.0, 80.0, 40.0, 10.0),   // 浮遊台3
    ];

    let start_time = std::time::Instant::now();

    loop {
        // 更新
        mario.update(&engine.input, camera.rotation, 1.0 / 60.0);
        camera.follow(&mario.position, 0.1);

        // カメラ回転（Q/E キー）
        if engine.input.btn(Key::A) {
            camera.rotate_left(0.05);
        }
        if engine.input.btn(Key::D) {
            camera.rotate_right(0.05);
        }

        // 投影器のカメラ角度を更新
        projector.set_camera_angle(camera.rotation * 180.0 / std::f32::consts::PI);

        // 描画
        engine.clear(1); // 青い背景

        // プラットフォーム描画
        for platform in &platforms {
            platform.draw(&mut engine.drawing, &projector);
        }

        // マリオ描画
        mario.draw(&mut engine.drawing, &projector);

        // UI
        engine.drawing.print("MARIO 64 - PHASE 13", 10, 5, 7);
        engine.drawing.print(&format!("POS: ({:.0}, {:.0}, {:.0})", mario.position.x, mario.position.y, mario.position.z), 5, 110, 7);
        engine.drawing.print(&format!("CAM: {:.0}°", camera.rotation * 180.0 / std::f32::consts::PI), 80, 110, 7);
        engine.drawing.print("Arrow/WASD:Move  Space:Jump", 5, 95, 7);
        engine.drawing.print("Q/E:Rotate Cam", 5, 102, 7);

        engine.update(1.0 / 60.0);
    }
}
