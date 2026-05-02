/// 数学ユーティリティ（3D座標、行列、投影など）

pub mod vec3;
pub mod isometric;

pub use vec3::Vec3;
pub use isometric::{IsometricProjector, IsoCamera};
