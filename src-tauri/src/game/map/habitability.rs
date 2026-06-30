use super::{MAP_WIDTH, MAP_HEIGHT, SLOPE_STEEP, W_ALT, W_SLP, MIN_HAB};

/// 预计算平坦区域中心度 (512×512, 0.0~1.0)
///
/// 基于宜居度栅格，对每个连通高宜居度区域，计算像素到区域边界的距离，在全局归一化。
/// 最大平坦区域的几何中心 = 1.0，边缘 = 0.0，小区域中心度按比例降低。
/// 方法: 宜居度阈值 → 高宜居度种子 → Chamfer distance → 全局最大值归一化
pub(super) fn compute_flat_center(habitability: &[f32]) -> Vec<f32> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;

    const INF: i32 = i32::MAX / 4;
    let threshold = MIN_HAB;

    // 1. 高宜居度区域掩码 & 边界种子初始化
    let mut flat = vec![false; total];
    for i in 0..total {
        flat[i] = habitability[i] >= threshold;
    }

    let mut dist = vec![INF; total];
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            if !flat[idx] { continue; }

            // 4-邻域检查是否位于平坦区域边界
            let on_boundary =
                (y > 0     && !flat[(y - 1) * w + x])
                || (y + 1 < h && !flat[(y + 1) * w + x])
                || (x > 0     && !flat[y * w + (x - 1)])
                || (x + 1 < w && !flat[y * w + (x + 1)]);

            if on_boundary {
                dist[idx] = 0; // 边界种子: 到自身距离为0
            }
        }
    }

    // 2. Chamfer distance transform (3-4 kernel) — 测量到高宜居度区域边界的距离
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let mut best = dist[idx];
            if y > 0 { best = best.min(dist[(y - 1) * w + x].saturating_add(3)); }
            if x > 0 { best = best.min(dist[y * w + (x - 1)].saturating_add(3)); }
            if y > 0 && x > 0 { best = best.min(dist[(y - 1) * w + (x - 1)].saturating_add(4)); }
            if y > 0 && x + 1 < w { best = best.min(dist[(y - 1) * w + (x + 1)].saturating_add(4)); }
            dist[idx] = best;
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let idx = y * w + x;
            let mut best = dist[idx];
            if y + 1 < h { best = best.min(dist[(y + 1) * w + x].saturating_add(3)); }
            if x + 1 < w { best = best.min(dist[y * w + (x + 1)].saturating_add(3)); }
            if y + 1 < h && x + 1 < w { best = best.min(dist[(y + 1) * w + (x + 1)].saturating_add(4)); }
            if y + 1 < h && x > 0 { best = best.min(dist[(y + 1) * w + (x - 1)].saturating_add(4)); }
            dist[idx] = best;
        }
    }

    // 3. 全局归一化：大面积平坦区域中心得分更高
    let mut flat_center = vec![0.0f32; total];
    let mut visited = vec![false; total];
    let mut components: Vec<Vec<usize>> = Vec::new();
    let mut global_max_d = 1; // 避免除零

    // 3a. 第一遍：收集所有连通分量，找出全局最大距离
    for start in 0..total {
        if !flat[start] || visited[start] { continue; }

        // BFS 收集连通分量
        let mut component: Vec<usize> = Vec::new();
        let mut queue: Vec<usize> = vec![start];
        visited[start] = true;

        while let Some(idx) = queue.pop() {
            component.push(idx);
            let (cy, cx) = (idx / w, idx % w);

            // 4-邻域入队
            if cy > 0     { let nxt = (cy - 1) * w + cx; if flat[nxt] && !visited[nxt] { visited[nxt] = true; queue.push(nxt); } }
            if cy + 1 < h { let nxt = (cy + 1) * w + cx; if flat[nxt] && !visited[nxt] { visited[nxt] = true; queue.push(nxt); } }
            if cx > 0     { let nxt = cy * w + (cx - 1); if flat[nxt] && !visited[nxt] { visited[nxt] = true; queue.push(nxt); } }
            if cx + 1 < w { let nxt = cy * w + (cx + 1); if flat[nxt] && !visited[nxt] { visited[nxt] = true; queue.push(nxt); } }
        }

        // 找分量内最大距离，同步更新全局最大值
        let max_d = component.iter()
            .filter_map(|&i| {
                let d = dist[i];
                if d < INF / 2 { Some(d) } else { None }
            })
            .max()
            .unwrap_or(0)
            .max(1);

        global_max_d = global_max_d.max(max_d);
        components.push(component);
    }

    // 3b. 第二遍：用全局最大距离归一化所有分量
    for component in &components {
        for &i in component {
            let d = dist[i];
            if d < INF / 2 {
                flat_center[i] = (d as f32) / (global_max_d as f32);
            } else {
                // 孤立平坦像素（无边界面可达）：仍给高分
                flat_center[i] = 1.0;
            }
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
