use super::{MAP_WIDTH, MAP_HEIGHT, SLOPE_STEEP, MAX_RAD_PX, W_ALT, W_SLP, W_RAD};

/// 预计算全图宜居度栅格 (512×512, 0.0~1.0)
///
/// 流程:
///   1. Sobel 梯度 → 坡度
///   2. 陡坡掩码
///   3. Chamfer distance transform → 平坦辐射半径
///   4. 加权合并: 海拔分 + 坡度分 + 辐射分
pub(super) fn compute_habitability(heights: &[f64]) -> Vec<f32> {
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

    // 4. 归一化 distance → 辐射分 (0..1)
    let max_rad = MAX_RAD_PX;
    // chamfer 3-4 kernel 的步长需要缩放:
    // 在 3-4 chamfer 中, 一个像素步 = 3 个单位, 对角步 = 4 个单位
    // 像素间距 ≈ 1, 所以实际距离 ≈ d / 3
    let scale = 3.0; // chamfer 步长归一化
    let mut rad_score = vec![0.0f32; total];
    for (i, &d) in dist.iter().enumerate() {
        if d >= INF / 2 {
            // 整片无陡坡可达 → 辐射半径视为最大 (满分)
            rad_score[i] = 1.0;
        } else {
            let real_dist = d as f64 / scale;
            let norm = (real_dist / max_rad).min(1.0);
            rad_score[i] = norm as f32;
        }
    }

    // 5. 合并三项
    let mut habitability = Vec::with_capacity(total);
    for i in 0..total {
        let h = heights[i];

        // 海拔分: 越低越好 (h 已归一化 0..1)
        let alt_score = 1.0 - h;

        // 坡度分: 越小越好
        let slp_norm = (slope[i] / steep_threshold).min(1.0);
        let slp_score = 1.0 - slp_norm;

        // 加权
        let score = W_ALT * alt_score + W_SLP * slp_score + W_RAD * rad_score[i] as f64;
        habitability.push(score.clamp(0.0, 1.0) as f32);
    }

    habitability
}
