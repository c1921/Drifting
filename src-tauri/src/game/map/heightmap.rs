use noise::{NoiseFn, Perlin};
use super::{MAP_WIDTH, MAP_HEIGHT, MAP_SCALE, OCTAVES, PERSISTENCE, LACUNARITY, WORLD_MIN, WORLD_MAX};

/// 使用 FBM Perlin 噪声生成高度网格，返回 0.0~1.0
pub(super) fn generate_heightmap(seed: u32) -> Vec<f64> {
    let perlin = Perlin::new(seed);

    let mut heights = Vec::with_capacity((MAP_WIDTH * MAP_HEIGHT) as usize);

    // 预计算常量（循环外一次）
    let half = (MAP_WIDTH / 2) as f64;
    let inv_max_dist = 1.0 / (half * half + half * half).sqrt();
    const MAX_AMP: f64 = 1.875; // Σ(PERSISTENCE^i), i=0..3 = 1 + 0.5 + 0.25 + 0.125

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let nx = x as f64 * MAP_SCALE;
            let ny = y as f64 * MAP_SCALE;

            let mut value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            for _ in 0..OCTAVES {
                value += perlin.get([nx * frequency, ny * frequency]) * amplitude;
                amplitude *= PERSISTENCE;
                frequency *= LACUNARITY;
            }

            // 归一化并施加对比度（始终在 [0, 1]）
            let clamped = ((value / MAX_AMP + 1.0) * 0.5).powf(3.0);

            // 盆地效果：边缘高、中心低
            let dx = x as f64 - half;
            let dy = y as f64 - half;
            let edge_factor = (dx * dx + dy * dy).sqrt() * inv_max_dist;

            let height =
                (clamped * (0.15 + edge_factor * 1.5) - (1.0 - edge_factor) * 0.10).max(0.0);

            heights.push(height);
        }
    }

    heights
}

/// 获取高度图上 (x, y) 处的高度值
#[allow(dead_code)]
pub(super) fn get_height(heights: &[f64], x: u32, y: u32) -> f64 {
    let idx = (y * MAP_WIDTH + x) as usize;
    heights[idx.min(heights.len() - 1)]
}

/// 将像素坐标转换为世界坐标（地图中心为 (0,0)）
pub(super) fn pixel_to_world(px: u32, py: u32) -> (f64, f64) {
    let wx = (px as f64 / MAP_WIDTH as f64) * (WORLD_MAX - WORLD_MIN) + WORLD_MIN;
    let wy = (py as f64 / MAP_HEIGHT as f64) * (WORLD_MAX - WORLD_MIN) + WORLD_MIN;
    (wx, wy)
}

/// 将世界坐标转换为像素坐标（四舍五入取整）
pub(super) fn world_to_pixel(wx: f64, wy: f64) -> (u32, u32) {
    let px = ((wx - WORLD_MIN) / (WORLD_MAX - WORLD_MIN) * MAP_WIDTH as f64).round() as u32;
    let py = ((wy - WORLD_MIN) / (WORLD_MAX - WORLD_MIN) * MAP_HEIGHT as f64).round() as u32;
    (px.clamp(0, MAP_WIDTH - 1), py.clamp(0, MAP_HEIGHT - 1))
}
