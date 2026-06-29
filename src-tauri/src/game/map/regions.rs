use super::{MAP_WIDTH, MAP_HEIGHT, MIN_HAB, REGION_DOMINANCE_RATIO};
use super::heightmap;

/// 地区数据：ID + 多边形轮廓
#[derive(Debug, Clone, serde::Serialize)]
pub struct RegionData {
    pub id: String,
    /// 世界坐标闭合环 [x0,y0,x1,y1,...]，与 boundary 格式一致
    pub polygon: Vec<f64>,
}

/// 计算地区划分
///
/// 1. 用 MIN_HAB 阈值生成高宜居度掩码
/// 2. 找出所有连通分量（每个分量直接作为一个自然区域）
/// 3. 过滤面积 < 总高宜居度面积 0.5% 的极小孤立碎片
/// 4. 拆分面积 ≥ 总高宜居度面积 50% 的占主导区域
/// 5. 为每个区域提取多边形轮廓
pub(super) fn compute_regions(habitability: &[f32], _seed: u32) -> Vec<RegionData> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;

    // 1. 高宜居度掩码
    let mut high_hab = vec![false; total];
    for i in 0..total {
        high_hab[i] = habitability[i] >= MIN_HAB;
    }

    // 2. 连通分量标记 — 每个连通分量直接作为一个自然区域
    let mut comp_ids = vec![-1i32; total];
    let mut components: Vec<Vec<usize>> = Vec::new();

    for start in 0..total {
        if !high_hab[start] || comp_ids[start] >= 0 {
            continue;
        }
        let cid = components.len() as i32;
        let mut stack = vec![start];
        comp_ids[start] = cid;
        let mut pixels = Vec::new();

        while let Some(idx) = stack.pop() {
            pixels.push(idx);
            let (cy, cx) = (idx / w, idx % w);

            if cy > 0 {
                let n = (cy - 1) * w + cx;
                if high_hab[n] && comp_ids[n] < 0 { comp_ids[n] = cid; stack.push(n); }
            }
            if cy + 1 < h {
                let n = (cy + 1) * w + cx;
                if high_hab[n] && comp_ids[n] < 0 { comp_ids[n] = cid; stack.push(n); }
            }
            if cx > 0 {
                let n = cy * w + (cx - 1);
                if high_hab[n] && comp_ids[n] < 0 { comp_ids[n] = cid; stack.push(n); }
            }
            if cx + 1 < w {
                let n = cy * w + (cx + 1);
                if high_hab[n] && comp_ids[n] < 0 { comp_ids[n] = cid; stack.push(n); }
            }
        }

        components.push(pixels);
    }

    if components.is_empty() {
        return Vec::new();
    }

    // 3. 过滤极小碎片：删除面积 < 总高宜居度面积 0.5% 的孤立分量
    let total_high_hab: usize = components.iter().map(|p| p.len()).sum();
    let min_component_area = (total_high_hab as f64 * 0.005) as usize;
    let mut region_pixels: Vec<Vec<usize>> = components
        .into_iter()
        .filter(|p| p.len() >= min_component_area)
        .collect();

    if region_pixels.is_empty() {
        return Vec::new();
    }

    // 4. 面积占比检查：如果某个区域占总高宜居度面积的 ≥50%，
    //     则将其拆分为多个子区域
    let total_high_hab: usize = region_pixels.iter().map(|p| p.len()).sum();
    let dominant_idx = region_pixels.iter().position(|p| (p.len() as f64) / (total_high_hab as f64) >= REGION_DOMINANCE_RATIO);

    if let Some(did) = dominant_idx {
        // 取出主导区域（保留其他区域不动）
        let dominant_pixels = region_pixels.swap_remove(did);

        // 计算需要拆分为几个子区域（使每个子区域面积占比 ≤20%）
        let area_ratio = dominant_pixels.len() as f64 / total_high_hab as f64;
        let split_count = (area_ratio / 0.05).ceil() as usize;
        let split_count = split_count.max(2); // 至少拆分为 2 个

        log::info!(
            "Splitting dominant region (ratio={:.2}) into {} sub-regions",
            area_ratio, split_count
        );

        // 在 dominant 区域内选 split_count 个种子（宜居度最高且距离较远）
        let seeds = pick_distant_seeds(&dominant_pixels, &habitability, w, split_count);

        // 重新 BFS 划分 dominant 区域
        let mut new_region_id = vec![-1i32; total];
        let mut dominant_mask = vec![false; total];
        for &p in &dominant_pixels { dominant_mask[p] = true; }

        let mut queue = std::collections::VecDeque::new();
        for (ri, &seed_idx) in seeds.iter().enumerate() {
            new_region_id[seed_idx] = ri as i32;
            queue.push_back(seed_idx);
        }

        while let Some(idx) = queue.pop_front() {
            let rid = new_region_id[idx];
            let (cy, cx) = (idx / w, idx % w);
            let neighbors = [
                if cy > 0     { Some((cy - 1) * w + cx) } else { None },
                if cy + 1 < h { Some((cy + 1) * w + cx) } else { None },
                if cx > 0     { Some(cy * w + (cx - 1)) } else { None },
                if cx + 1 < w { Some(cy * w + (cx + 1)) } else { None },
            ];
            for nb in neighbors.iter().flatten() {
                if dominant_mask[*nb] && new_region_id[*nb] < 0 {
                    new_region_id[*nb] = rid;
                    queue.push_back(*nb);
                }
            }
        }

        // 重建拆分后的子区域，追加到 region_pixels
        for si in 0..split_count {
            let sub_pixels: Vec<usize> = dominant_pixels.iter()
                .filter(|&&p| new_region_id[p] == si as i32)
                .copied()
                .collect();
            if !sub_pixels.is_empty() {
                region_pixels.push(sub_pixels);
            }
        }
    }

    // 5. 为每个区域提取多边形轮廓
    let region_count = region_pixels.len();
    let mut region_masks: Vec<Vec<bool>> = (0..region_count).map(|_| vec![false; total]).collect();
    for (ri, pixels) in region_pixels.iter().enumerate() {
        for &p in pixels {
            region_masks[ri][p] = true;
        }
    }

    let mut regions: Vec<RegionData> = Vec::new();
    for (ri, mask) in region_masks.iter().enumerate() {
        let polygon = extract_polygon_outline(mask, w, h);
        regions.push(RegionData {
            id: format!("region_{}", ri + 1),
            polygon,
        });
    }

    regions
}

/// 从 dominant 区域内选取指定数量的种子点
///
/// 优先选宜居度最高且彼此远离的像素，确保拆分后各子区域空间合理。
fn pick_distant_seeds(pixels: &[usize], habitability: &[f32], w: usize, count: usize) -> Vec<usize> {
    if pixels.len() <= count {
        // 像素数不够，直接取所有
        return pixels.to_vec();
    }

    // 按宜居度排序（降序），取前 20% 作为候选池
    let mut candidates: Vec<usize> = pixels.to_vec();
    candidates.sort_by(|&a, &b| habitability[b].partial_cmp(&habitability[a]).unwrap());
    let pool_size = (candidates.len() / 5).max(count * 3);
    candidates.truncate(pool_size);

    let mut seeds: Vec<usize> = Vec::new();

    // 取最高的作为第一个种子
    seeds.push(candidates[0]);

    let (w_f64, h_f64) = (MAP_WIDTH as f64, MAP_HEIGHT as f64);
    let max_dist = (w_f64 * w_f64 + h_f64 * h_f64).sqrt();

    while seeds.len() < count && seeds.len() < candidates.len() {
        let mut best_idx = 0;
        let mut best_score = -1.0f64;

        for &p in &candidates {
            if seeds.contains(&p) { continue; }

            let px = (p % w) as f64;
            let py = (p / w) as f64;

            // 到所有已选种子的最小曼哈顿距离
            let min_dist = seeds.iter()
                .map(|&s| {
                    let sx = (s % w) as f64;
                    let sy = (s / w) as f64;
                    let dx = px - sx;
                    let dy = py - sy;
                    (dx * dx + dy * dy).sqrt()
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0);

            // 评分：宜居度 × 距离因子（归一化后开方）
            let dist_factor = (min_dist / max_dist).sqrt();
            let score = (habitability[p] as f64) * dist_factor;

            if score > best_score {
                best_score = score;
                best_idx = p;
            }
        }

        seeds.push(best_idx);
    }

    seeds
}

/// 从布尔掩码中提取多边形轮廓（像素坐标 → 世界坐标）
///
/// 算法：扫描每个像素，检查 4 条边是否在掩码边界上，
/// 收集所有边界边，连接成环。
fn extract_polygon_outline(mask: &[bool], w: usize, h: usize) -> Vec<f64> {
    // 收集所有边界线段（水平/垂直）
    let mut segments: Vec<((f64, f64), (f64, f64))> = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            if !mask[idx] { continue; }

            // 上边 (y, x) -> (y, x+1)
            if y == 0 || !mask[(y - 1) * w + x] {
                segments.push(((x as f64, y as f64), ((x + 1) as f64, y as f64)));
            }
            // 下边 (y+1, x) -> (y+1, x+1)
            if y + 1 == h || !mask[(y + 1) * w + x] {
                segments.push(((x as f64, (y + 1) as f64), ((x + 1) as f64, (y + 1) as f64)));
            }
            // 左边 (y, x) -> (y+1, x)
            if x == 0 || !mask[y * w + (x - 1)] {
                segments.push(((x as f64, y as f64), (x as f64, (y + 1) as f64)));
            }
            // 右边 (y, x+1) -> (y+1, x+1)
            if x + 1 == w || !mask[y * w + (x + 1)] {
                segments.push((((x + 1) as f64, y as f64), ((x + 1) as f64, (y + 1) as f64)));
            }
        }
    }

    // 将线段连接成环（贪心算法）
    let mut polygon: Vec<(f64, f64)> = Vec::new();
    if segments.is_empty() {
        return polygon.into_iter().flat_map(|(x, y)| vec![x, y]).collect();
    }

    // 取第一条线段
    let (current_start, current_end) = segments.remove(0);
    polygon.push(current_start);
    polygon.push(current_end);

    while !segments.is_empty() {
        let last = polygon.last().copied().unwrap();
        let mut found = false;

        for i in (0..segments.len()).rev() {
            let (s, e) = &segments[i];
            // 检查 end 是否等于 last
            if approx_eq(s.0, last.0) && approx_eq(s.1, last.1) {
                polygon.push(*e);
                segments.swap_remove(i);
                found = true;
                break;
            }
            if approx_eq(e.0, last.0) && approx_eq(e.1, last.1) {
                polygon.push(*s);
                segments.swap_remove(i);
                found = true;
                break;
            }
        }

        if !found {
            // 应该是闭合了，或者数据异常
            break;
        }
    }

    // 像素坐标 → 世界坐标
    let world_poly: Vec<f64> = polygon.iter()
        .flat_map(|&(px, py)| {
            let (wx, wy) = heightmap::pixel_to_world(px as u32, py as u32);
            vec![wx, wy]
        })
        .collect();

    world_poly
}

/// 浮点数近似相等
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.001
}
