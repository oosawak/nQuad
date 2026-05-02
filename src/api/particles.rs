/// パーティクルシステム
///
/// 物理演算ベースのパーティクルエフェクト管理

use crate::api::drawing::DrawingContext;

/// 単一パーティクル
#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub dx: f32,      // 速度X
    pub dy: f32,      // 速度Y
    pub color: u8,    // パレットインデックス
    pub life: u32,    // 残りライフ（フレーム数）
}

impl Particle {
    /// 新規パーティクルを作成
    pub fn new(x: f32, y: f32, dx: f32, dy: f32, color: u8, lifetime: u32) -> Self {
        Particle {
            x,
            y,
            dx,
            dy,
            color,
            life: lifetime,
        }
    }

    /// パーティクルを更新
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dy += 0.2;               // 重力
        self.life = self.life.saturating_sub(1);
    }

    /// パーティクルが生存しているか判定
    pub fn is_alive(&self) -> bool {
        self.life > 0
    }
}

/// パーティクルシステム
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    max_particles: usize,
    default_lifetime: u32,
}

impl ParticleSystem {
    /// 新規パーティクルシステムを作成
    ///
    /// # 引数
    /// - `capacity`: 最大パーティクル数
    pub fn new(capacity: usize) -> Self {
        Self {
            particles: Vec::new(),
            max_particles: capacity,
            default_lifetime: 30,  // デフォルト 30フレーム
        }
    }

    /// パーティクルを生成
    ///
    /// # 引数
    /// - `x`: X座標
    /// - `y`: Y座標
    /// - `dx`: X速度
    /// - `dy`: Y速度
    /// - `color`: パレットインデックス
    pub fn emit(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: u8) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle::new(x, y, dx, dy, color, self.default_lifetime));
        }
    }

    /// カスタムライフタイムでパーティクルを生成
    pub fn emit_with_lifetime(&mut self, x: f32, y: f32, dx: f32, dy: f32, color: u8, lifetime: u32) {
        if self.particles.len() < self.max_particles {
            self.particles.push(Particle::new(x, y, dx, dy, color, lifetime));
        }
    }

    /// すべてのパーティクルを更新
    pub fn update(&mut self) {
        self.particles.retain_mut(|p| {
            p.update();
            p.is_alive()
        });
    }

    /// すべてのパーティクルを描画
    pub fn draw(&self, ctx: &mut DrawingContext) {
        for p in &self.particles {
            ctx.pset(p.x as i32, p.y as i32, p.color);
        }
    }

    /// パーティクル数を取得
    pub fn count(&self) -> usize {
        self.particles.len()
    }

    /// デフォルトライフタイムを設定
    pub fn set_default_lifetime(&mut self, lifetime: u32) {
        self.default_lifetime = lifetime;
    }

    /// すべてのパーティクルをクリア
    pub fn clear(&mut self) {
        self.particles.clear();
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new(256) // デフォルト最大数 256
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let p = Particle::new(10.0, 20.0, 1.0, 1.0, 5, 30);
        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 20.0);
        assert_eq!(p.color, 5);
        assert_eq!(p.life, 30);
        assert!(p.is_alive());
    }

    #[test]
    fn test_particle_update() {
        let mut p = Particle::new(0.0, 0.0, 1.0, 1.0, 5, 30);
        p.update();
        
        assert_eq!(p.x, 1.0);
        assert!(p.y > 1.0); // 重力で加速
        assert!(p.dy > 1.0);
        assert_eq!(p.life, 29);
    }

    #[test]
    fn test_particle_lifetime() {
        let mut p = Particle::new(0.0, 0.0, 0.0, 0.0, 5, 3);
        
        assert!(p.is_alive());
        p.update();
        assert_eq!(p.life, 2);
        assert!(p.is_alive());
        
        p.update();
        p.update();
        assert_eq!(p.life, 0);
        assert!(!p.is_alive());
    }

    #[test]
    fn test_particle_system_emit() {
        let mut system = ParticleSystem::new(10);
        
        system.emit(5.0, 5.0, 1.0, 1.0, 3);
        assert_eq!(system.count(), 1);
        
        system.emit(10.0, 10.0, -1.0, -1.0, 8);
        assert_eq!(system.count(), 2);
    }

    #[test]
    fn test_particle_system_max_capacity() {
        let mut system = ParticleSystem::new(3);
        
        for i in 0..5 {
            system.emit(i as f32, i as f32, 0.0, 0.0, 7);
        }
        
        // 容量を超えないように制限される
        assert_eq!(system.count(), 3);
    }

    #[test]
    fn test_particle_system_update_and_cleanup() {
        let mut system = ParticleSystem::new(10);
        
        system.emit(0.0, 0.0, 0.0, 0.0, 3);
        system.emit_with_lifetime(10.0, 10.0, 0.0, 0.0, 5, 1);
        
        assert_eq!(system.count(), 2);
        
        system.update();
        assert_eq!(system.count(), 2);
        
        system.update(); // 1フレーム寿命のパーティクルが死ぬ
        assert_eq!(system.count(), 1);
    }

    #[test]
    fn test_particle_system_clear() {
        let mut system = ParticleSystem::new(10);
        
        for _ in 0..5 {
            system.emit(0.0, 0.0, 0.0, 0.0, 7);
        }
        
        assert_eq!(system.count(), 5);
        system.clear();
        assert_eq!(system.count(), 0);
    }
}
