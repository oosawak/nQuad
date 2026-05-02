//! Mario Kart Style Racing Game
//!
//! A simple top-down racing game with:
//! - Player-controlled kart with acceleration, braking, and drift
//! - Dynamic speed visualization
//! - Track collision detection
//! - Camera follow system
//! - Speedometer and lap tracking

use nantaraquad::api::drawing::DrawingContext;
use nantaraquad::api::input::{InputState, Key};
use nantaraquad::api::game::GameEngine;
use std::time::Instant;

const TRACK_WIDTH: u32 = 400;
const TRACK_HEIGHT: u32 = 300;
const KART_WIDTH: u32 = 6;
const KART_HEIGHT: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Title,
    Racing,
    Finished,
}

struct Kart {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    speed: f32,
    angle: f32,
    width: u32,
    height: u32,
    is_drifting: bool,
    drift_counter: u32,
}

impl Kart {
    fn new(x: f32, y: f32) -> Self {
        Kart {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            speed: 0.0,
            angle: 0.0,
            width: KART_WIDTH,
            height: KART_HEIGHT,
            is_drifting: false,
            drift_counter: 0,
        }
    }

    fn update(&mut self, input: &InputState, dt: f32) {
        const MAX_SPEED: f32 = 150.0;
        const ACCELERATION: f32 = 200.0;
        const FRICTION: f32 = 0.9;
        const TURN_SPEED: f32 = 3.0;

        // アクセル
        let mut accel = 0.0;
        if input.btn(Key::Up) || input.btn(Key::W) {
            accel = ACCELERATION;
        }

        // ブレーキ
        if input.btn(Key::Down) || input.btn(Key::S) {
            accel = -ACCELERATION * 0.5;
        }

        // スピード更新
        self.speed += accel * dt;
        if self.speed > MAX_SPEED {
            self.speed = MAX_SPEED;
        }
        if self.speed < 0.0 {
            self.speed = 0.0;
        }

        // ハンドル
        if input.btn(Key::Left) || input.btn(Key::A) {
            self.angle -= TURN_SPEED * dt * (self.speed / MAX_SPEED).max(0.3);
        }
        if input.btn(Key::Right) || input.btn(Key::D) {
            self.angle += TURN_SPEED * dt * (self.speed / MAX_SPEED).max(0.3);
        }

        // ドリフト（スペースキー）
        if input.btnp(Key::Space) {
            self.is_drifting = !self.is_drifting;
            if self.is_drifting {
                self.drift_counter = 30;
            }
        }

        if self.is_drifting && self.drift_counter > 0 {
            self.drift_counter -= 1;
        } else if self.drift_counter == 0 {
            self.is_drifting = false;
        }

        // ドリフト時は加速する
        if self.is_drifting {
            self.speed = (self.speed + 30.0 * dt).min(MAX_SPEED);
        }

        // 摩擦
        self.speed *= FRICTION.powf(dt);

        // 位置更新（角度に基づいた方向）
        self.vx = self.angle.cos() * self.speed;
        self.vy = self.angle.sin() * self.speed;

        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }

    fn collide_with_track(&mut self, track: &Track) {
        // トラック内の通路に衝突判定
        if !track.is_on_road(self.x, self.y) {
            // 壁に衝突したら速度を減らす
            self.speed *= 0.5;

            // 前のフレームの位置に戻す
            self.x -= self.vx * 0.016;
            self.y -= self.vy * 0.016;
        }
    }

    fn draw(&self, drawing: &mut DrawingContext) {
        // カートの本体（方向付き）
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();

        // カートの色（ドリフト中は黄色）
        let color = if self.is_drifting { 6 } else { 4 }; // 4=赤, 6=黄

        // シンプルな矩形で描画
        let x = self.x as i32;
        let y = self.y as i32;
        drawing.rectfill(x - 3, y - 4, 6, 8, color);

        // 前方向インジケーター
        let front_x = x as f32 + cos_a * 6.0;
        let front_y = y as f32 + sin_a * 6.0;
        drawing.pset(front_x as i32, front_y as i32, 7); // 白点
    }
}

struct Track {
    start_x: f32,
    start_y: f32,
    // トラック形状（シンプルな長方形コース）
}

impl Track {
    fn new() -> Self {
        Track {
            start_x: 50.0,
            start_y: 50.0,
        }
    }

    fn is_on_road(&self, x: f32, y: f32) -> bool {
        // 道幅のチェック（左コーナーと右コーナーで異なる）
        let road_width = 60.0;
        let inner_margin = 30.0;

        // トラック部分のチェック
        if x > self.start_x - inner_margin
            && x < self.start_x + TRACK_WIDTH as f32 + inner_margin
            && y > self.start_y - inner_margin
            && y < self.start_y + TRACK_HEIGHT as f32 + inner_margin
        {
            // コーナーを避ける
            let corner_size = 50.0;

            // 左上コーナー
            if x < self.start_x + corner_size && y < self.start_y + corner_size {
                let dx = x - (self.start_x + corner_size);
                let dy = y - (self.start_y + corner_size);
                if dx * dx + dy * dy > corner_size * corner_size {
                    return false;
                }
            }

            // 右上コーナー
            let right_x = self.start_x + TRACK_WIDTH as f32;
            if x > right_x - corner_size && y < self.start_y + corner_size {
                let dx = x - (right_x - corner_size);
                let dy = y - (self.start_y + corner_size);
                if dx * dx + dy * dy > corner_size * corner_size {
                    return false;
                }
            }

            // 左下コーナー
            let bottom_y = self.start_y + TRACK_HEIGHT as f32;
            if x < self.start_x + corner_size && y > bottom_y - corner_size {
                let dx = x - (self.start_x + corner_size);
                let dy = y - (bottom_y - corner_size);
                if dx * dx + dy * dy > corner_size * corner_size {
                    return false;
                }
            }

            // 右下コーナー
            if x > right_x - corner_size && y > bottom_y - corner_size {
                let dx = x - (right_x - corner_size);
                let dy = y - (bottom_y - corner_size);
                if dx * dx + dy * dy > corner_size * corner_size {
                    return false;
                }
            }

            return true;
        }

        false
    }

    fn draw(&self, drawing: &mut DrawingContext) {
        // 背景（草）
        drawing.cls(2); // 緑

        // トラック本体（灰色）
        drawing.rectfill(
            self.start_x as i32,
            self.start_y as i32,
            TRACK_WIDTH,
            TRACK_HEIGHT,
            8, // 灰色
        );

        // コーナーマーカー（赤）
        let corner_size = 50;
        drawing.circfill(self.start_x as i32 + corner_size, self.start_y as i32 + corner_size, 8, 4);
        drawing.circfill(
            (self.start_x + TRACK_WIDTH as f32 - corner_size as f32) as i32,
            self.start_y as i32 + corner_size,
            8,
            4,
        );
        drawing.circfill(
            self.start_x as i32 + corner_size,
            (self.start_y + TRACK_HEIGHT as f32 - corner_size as f32) as i32,
            8,
            4,
        );
        drawing.circfill(
            (self.start_x + TRACK_WIDTH as f32 - corner_size as f32) as i32,
            (self.start_y + TRACK_HEIGHT as f32 - corner_size as f32) as i32,
            8,
            4,
        );

        // スタート/ゴール地点（黄色い線）
        drawing.line(
            self.start_x as i32,
            self.start_y as i32 - 5,
            self.start_x as i32,
            self.start_y as i32 + 5,
            6,
        );
    }
}

fn main() {
    let mut engine = GameEngine::new(160, 120, 60);
    let mut game_state = GameState::Title;
    let mut kart = Kart::new(60.0, 80.0);
    let track = Track::new();

    let mut lap_count = 0;
    let mut lap_start_time = Instant::now();
    let mut best_lap_time = std::f32::INFINITY;
    let mut last_checkpoint_x = track.start_x;

    let start_time = Instant::now();

    loop {
        match game_state {
            GameState::Title => {
                engine.clear(0); // 黒
                engine.drawing.print("MARIO KART", 50, 30, 7);
                engine.drawing.print("RACING", 65, 40, 7);
                engine.drawing.print("Press SPACE to Start", 20, 70, 7);
                engine.drawing.print("Arrow Keys or WASD: Move", 15, 85, 7);
                engine.drawing.print("Space: Drift", 50, 100, 7);

                if engine.input.btnp(Key::Space) {
                    game_state = GameState::Racing;
                    lap_start_time = Instant::now();
                }
            }
            GameState::Racing => {
                // 更新
                kart.update(&engine.input, 1.0 / 60.0);
                kart.collide_with_track(&track);

                // ラップ検出
                if kart.x < track.start_x + 10.0 && last_checkpoint_x >= track.start_x + 10.0 {
                    lap_count += 1;
                    let lap_time = lap_start_time.elapsed().as_secs_f32();
                    if lap_time < best_lap_time {
                        best_lap_time = lap_time;
                    }
                    lap_start_time = Instant::now();

                    if lap_count >= 3 {
                        game_state = GameState::Finished;
                    }
                }
                last_checkpoint_x = kart.x;

                // 描画
                engine.clear(0);
                track.draw(&mut engine.drawing);

                // パーティクル（スピードラインのような効果）
                if kart.is_drifting && kart.drift_counter % 2 == 0 {
                    let particle_angle = kart.angle + std::f32::consts::PI; // 後ろ方向
                    let speed = 30.0;
                    engine.particles.emit(
                        kart.x,
                        kart.y,
                        particle_angle.cos() * speed,
                        particle_angle.sin() * speed,
                        13, // ピンク
                    );
                }
                engine.particles.update();

                // カートの描画より前にパーティクルを描画
                engine.particles.draw(&mut engine.drawing);

                kart.draw(&mut engine.drawing);

                // UI: スピードメーター
                let speed_percentage = (kart.speed / 150.0 * 50.0) as u32;
                engine.drawing.print("SPEED", 5, 5, 7);
                engine.drawing.rect(5, 12, 50, 6, 7);
                if speed_percentage > 0 {
                    engine.drawing.rectfill(6, 13, speed_percentage, 4, 3); // 水色
                }

                // UI: ラップ表示
                engine.drawing.print(&format!("LAP: {}/3", lap_count), 100, 5, 7);

                // UI: ラップタイム
                let current_lap_time = lap_start_time.elapsed().as_secs_f32();
                engine.drawing.print(
                    &format!("TIME: {:.1}s", current_lap_time),
                    95, 15,
                    7,
                );

                // UI: ベストラップ
                if best_lap_time < std::f32::INFINITY {
                    engine.drawing.print(
                        &format!("BEST: {:.1}s", best_lap_time),
                        95, 25,
                        7,
                    );
                }

                // ドリフト状態表示
                if kart.is_drifting {
                    engine.drawing.print("DRIFT!", 65, 100, 6); // 黄色
                }
            }
            GameState::Finished => {
                engine.clear(0);
                engine.drawing.print("RACE FINISHED!", 40, 30, 7);
                engine.drawing.print(&format!("LAPS: {}", lap_count), 60, 50, 7);
                if best_lap_time < std::f32::INFINITY {
                    engine.drawing.print(
                        &format!("BEST LAP: {:.1}s", best_lap_time),
                        30, 65,
                        7,
                    );
                }
                engine.drawing.print("Press SPACE for Title", 20, 100, 7);

                if engine.input.btnp(Key::Space) {
                    game_state = GameState::Title;
                    kart = Kart::new(60.0, 80.0);
                    lap_count = 0;
                    best_lap_time = std::f32::INFINITY;
                }
            }
        }

        engine.update(1.0 / 60.0);
    }
}
