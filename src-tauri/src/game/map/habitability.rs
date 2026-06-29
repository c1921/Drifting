use super::{MAP_WIDTH, MAP_HEIGHT, SLOPE_STEEP, MAX_RAD_PX, W_ALT, W_SLP};

/// 预计算平坦区域中心度 (512×512, 0.0~1.0)
///
/// 值越高表示像素位于越大的平坦区域的中心位置。
/// 通过 Sobel 梯度 → 坡度 → 陡坡掩码 → Chamfer distance transform 计算。
pub(super) fn compute_flat_center(heights: &[f64]) -> Vec<f32> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;

    // 1. Sobel 梯度幅值
    let mut slope = vec![0.0f64; total];
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;

            // Sobel X
            let mut gx = 0.0;
            if x > 0 && x < w - 1 {
                let row_up = if y > 0 { y - 1 } else { y };
                let row_dn = if y + 1 < h { y + 1 } else { y };
                gx = heights[row_up * w + x + 1] * -1.0
                    + heights[row_up * w + x - 1] * 1.0
                    + heights[y * w + x + 1] * -2.0
                    + heights[y * w + x - 1] * 2.0
                    + heights[row_dn * w + x + 1] * -1.0
                    + heights[row_dn * w + x - 1] * 1.0;
                gx /= 4.0; // normalize
            }

            // Sobel Y
            let mut gy = 0.0;
            if y > 0 && y < h - 1 {
                let col_l = if x > 0 { x - 1 } else { x };
                let col_r = if x + 1 < w { x + 1 } else { x };
                gy = heights[(y - 1) * w + col_l] * 1.0
                    + heights[(y - 1) * w + x] * 2.0
                    + heights[(y - 1) * w + col_r] * 1.0
                    + heights[(y + 1) * w + col_l] * -1.0
                    + heights[(y + 1) * w + x] * -2.0
                    + heights[(y + 1) * w + col_r] * -1.0;
                gy /= 4.0;
            }

            slope[idx] = (gx * gx + gy * gy).sqrt();
        }
    }

    // 2. 陡坡掩码 & 距离变换初始化
    const INF: i32 = i32::MAX / 4;
    let steep_threshold = SLOPE_STEEP;
    let mut dist = vec![INF; total];
    for (i, &s) in slope.iter().enumerate() {
        if s > steep_threshold {
            dist[i] = 0; // 种子点: 陡坡自身距离为0
        }
    }

    // 3. Chamfer distance transform (3-4 kernel)
    //    前向: 左上 → 右下
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let mut best = dist[idx];
            // 正北 (y-1, x)
            if y > 0 {
                best = best.min(dist[(y - 1) * w + x].saturating_add(3));
            }
            // 正西 (y, x-1)
            if x > 0 {
                best = best.min(dist[y * w + (x - 1)].saturating_add(3));
            }
            // 西北 (y-1, x-1)
            if y > 0 && x > 0 {
                best = best.min(dist[(y - 1) * w + (x - 1)].saturating_add(4));
            }
            // 东北 (y-1, x+1)
            if y > 0 && x + 1 < w {
                best = best.min(dist[(y - 1) * w + (x + 1)].saturating_add(4));
            }
            dist[idx] = best;
        }
    }

    //    后向: 右下 → 左上
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let idx = y * w + x;
            let mut best = dist[idx];
            // 正南 (y+1, x)
            if y + 1 < h {
                best = best.min(dist[(y + 1) * w + x].saturating_add(3));
            }
            // 正东 (y, x+1)
            if x + 1 < w {
                best = best.min(dist[y * w + (x + 1)].saturating_add(3));
            }
            // 东南 (y+1, x+1)
            if y + 1 < h && x + 1 < w {
                best = best.min(dist[(y + 1) * w + (x + 1)].saturating_add(4));
            }
            // 西南 (y+1, x-1)
            if y + 1 < h && x > 0 {
                best = best.min(dist[(y + 1) * w + (x - 1)].saturating_add(4));
            }
            dist[idx] = best;
        }
    }

    // 4. 归一化 distance → 平坦中心度 (0..1)
    let max_rad = MAX_RAD_PX;
    let scale = 3.0;
    let mut flat_center = vec![0.0f32; total];
    for (i, &d) in dist.iter().enumerate() {
        if d >= INF / 2 {
            flat_center[i] = 1.0;
        } else {
            let real_dist = d as f64 / scale;
            let norm = (real_dist / max_rad).min(1.0);
            flat_center[i] = norm as f32;
        }
    }

    flat_center
}

/// 预计算全图宜居度栅格 (512×512, 0.0~1.0)
///
/// 仅基于地形因素：海拔分 + 坡度分（不含平坦中心度）。
/// 归一化到 0~1 范围，供所有地点（城市/小镇/村庄）作为优先级和硬阈值使用。
pub(super) fn compute_habitability(heights: &[f64]) -> Vec<f32> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;
    let w_sum = W_ALT + W_SLP; // 归一化分母

    // 1. Sobel 梯度幅值
    let mut slope = vec![0.0f64; total];
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;

            let mut gx = 0.0;
            if x > 0 && x < w - 1 {
                let row_up = if y > 0 { y - 1 } else { y };
                let row_dn = if y + 1 < h { y + 1 } else { y };
                gx = heights[row_up * w + x + 1] * -1.0
                    + heights[row_up * w + x - 1] * 1.0
                    + heights[y * w + x + 1] * -2.0
                    + heights[y * w + x - 1] * 2.0
                    + heights[row_dn * w + x + 1] * -1.0
                    + heights[row_dn * w + x - 1] * 1.0;
                gx /= 4.0;
            }

            let mut gy = 0.0;
            if y > 0 && y < h - 1 {
                let col_l = if x > 0 { x - 1 } else { x };
                let col_r = if x + 1 < w { x + 1 } else { x };
                gy = heights[(y - 1) * w + col_l] * 1.0
                    + heights[(y - 1) * w + x] * 2.0
                    + heights[(y - 1) * w + col_r] * 1.0
                    + heights[(y + 1) * w + col_l] * -1.0
                    + heights[(y + 1) * w + x] * -2.0
                    + heights[(y + 1) * w + col_r] * -1.0;
                gy /= 4.0;
            }

            slope[idx] = (gx * gx + gy * gy).sqrt();
        }
    }

    // 2. 合并：海拔分 + 坡度分（归一化到 0~1）
    let steep_threshold = SLOPE_STEEP;
    let mut habitability = Vec::with_capacity(total);
    for i in 0..total {
        let h = heights[i];

        // 海拔分: 越低越好 (h 已归一化 0..1)
        let alt_score = 1.0 - h;

        // 坡度分: 越小越好
        let slp_norm = (slope[i] / steep_threshold).min(1.0);
        let slp_score = 1.0 - slp_norm;

        // 归一化加权（仅海拔 + 坡度）
        let score = (W_ALT * alt_score + W_SLP * slp_score) / w_sum;
        habitability.push(score.clamp(0.0, 1.0) as f32);
    }

    habitability
}
