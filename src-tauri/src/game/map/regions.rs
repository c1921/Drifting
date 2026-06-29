use super::{MAP_WIDTH, MAP_HEIGHT, CITY_COUNT, MIN_HAB, REGION_INITIAL_COUNT, REGION_DOMINANCE_RATIO};
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
/// 2. 找出所有连通分量
/// 3. 每个分量取宜居度最高点作为种子
/// 4. BFS 从种子向外辐射，遇到低宜居度停止
/// 5. 反复合并最小相邻区域，直到数量 == CITY_COUNT
pub(super) fn compute_regions(habitability: &[f32], _seed: u32) -> Vec<RegionData> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;

    // 1. 高宜居度掩码
    let mut high_hab = vec![false; total];
    for i in 0..total {
        high_hab[i] = habitability[i] >= MIN_HAB;
    }

    // 2. 连通分量标记
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

            // 4-邻域
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

    // 3. 每个分量取宜居度最高的像素作为种子
    //    可用种子数 = min(REGION_INITIAL_COUNT, 分量数)
    let num_seeds = components.len().min(REGION_INITIAL_COUNT);

    // 对分量按宜居度最高像素排序（降序），取前 num_seeds 个
    let mut comp_scores: Vec<(usize, f32)> = components.iter().enumerate()
        .map(|(ci, pixels)| {
            let max_hab = pixels.iter().map(|&p| habitability[p]).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0);
            (ci, max_hab)
        })
        .collect();
    comp_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 取前 num_seeds 个分量作为种子源
    let chosen: Vec<usize> = comp_scores.iter().take(num_seeds).map(|&(ci, _)| ci).collect();
    // 剩余分量标记为"待分配"，稍后合并到相邻区域
    let remaining_comps: Vec<usize> = (0..components.len()).filter(|i| !chosen.contains(i)).collect();

    // 为每个选中的分量选种子点（分量内宜居度最高像素）
    let seeds: Vec<(usize, usize)> = chosen.iter().map(|&ci| {
        let pixels = &components[ci];
        let best_idx = pixels.iter()
            .max_by(|&&a, &&b| habitability[a].partial_cmp(&habitability[b]).unwrap())
            .copied()
            .unwrap_or(pixels[0]);
        (ci, best_idx)
    }).collect();

    // 4. BFS 辐射分配 — 每个像素归入最先到达它的种子
    //    使用多源 BFS，每个种子一个区域
    let mut region_id = vec![-1i32; total];
    let mut queue = std::collections::VecDeque::new();

    for (ri, &(_, seed_idx)) in seeds.iter().enumerate() {
        region_id[seed_idx] = ri as i32;
        queue.push_back(seed_idx);
    }

    while let Some(idx) = queue.pop_front() {
        let rid = region_id[idx];
        let (cy, cx) = (idx / w, idx % w);

        let neighbors = [
            if cy > 0     { Some((cy - 1) * w + cx) } else { None },
            if cy + 1 < h { Some((cy + 1) * w + cx) } else { None },
            if cx > 0     { Some(cy * w + (cx - 1)) } else { None },
            if cx + 1 < w { Some(cy * w + (cx + 1)) } else { None },
        ];

        for nb in neighbors.iter().flatten() {
            if high_hab[*nb] && region_id[*nb] < 0 {
                region_id[*nb] = rid;
                queue.push_back(*nb);
            }
        }
    }

    // 将剩余分量（未选中的）分配给最近的已分配种子
    // 通过 BFS 从已分配区域向外扩展，覆盖剩余 high_hab 像素
    // 用第二遍 BFS 覆盖所有尚未分配的像素（包括剩余分量）
    let mut extended_queue = std::collections::VecDeque::new();
    // 先收集所有已分配像素
    for i in 0..total {
        if region_id[i] >= 0 {
            extended_queue.push_back(i);
        }
    }
    while let Some(idx) = extended_queue.pop_front() {
        let rid = region_id[idx];
        let (cy, cx) = (idx / w, idx % w);

        let neighbors = [
            if cy > 0     { Some((cy - 1) * w + cx) } else { None },
            if cy + 1 < h { Some((cy + 1) * w + cx) } else { None },
            if cx > 0     { Some(cy * w + (cx - 1)) } else { None },
            if cx + 1 < w { Some(cy * w + (cx + 1)) } else { None },
        ];

        for nb in neighbors.iter().flatten() {
            if high_hab[*nb] && region_id[*nb] < 0 {
                region_id[*nb] = rid;
                extended_queue.push_back(*nb);
            }
        }
    }

    // 清理剩余分量引用
    drop(remaining_comps);

    // 构建区域像素列表
    let mut region_pixels: Vec<Vec<usize>> = (0..seeds.len()).map(|_| Vec::new()).collect();
    for i in 0..total {
        let rid = region_id[i];
        if rid >= 0 && (rid as usize) < region_pixels.len() {
            region_pixels[rid as usize].push(i);
        }
    }

    // 过滤掉空区域
    region_pixels.retain(|p| !p.is_empty());

    // 5. 合并小区域直到数量 == CITY_COUNT
    while region_pixels.len() > CITY_COUNT {
        // 找到面积最小的区域
        let smallest = (0..region_pixels.len())
            .min_by_key(|&i| region_pixels[i].len())
            .unwrap();

        // 找到与该区域相邻的最大区域
        let adj_sizes: Vec<(usize, usize)> = (0..region_pixels.len())
            .filter(|&i| i != smallest && regions_adjacent(&region_pixels, i, smallest, w, h))
            .map(|i| (i, region_pixels[i].len()))
            .collect();

        if adj_sizes.is_empty() {
            // 没有相邻区域，删除该区域
            region_pixels.remove(smallest);
            continue;
        }

        let (largest_adj, _) = adj_sizes.iter()
            .max_by_key(|&&(_, sz)| sz)
            .copied()
            .unwrap();

        // 合并 smallest 到 largest_adj
        let pixels = region_pixels.remove(smallest);
        let target = if largest_adj < smallest { largest_adj } else { largest_adj - 1 };
        region_pixels[target].extend(pixels);
    }

    // 5b. 面积占比检查：如果一个区域占总高宜居度面积的 ≥80%，
    //     则舍弃其余小区域，将大区域拆分为 CITY_COUNT 个子区域
    let total_high_hab: usize = region_pixels.iter().map(|p| p.len()).sum();
    let dominant_idx = region_pixels.iter().position(|p| (p.len() as f64) / (total_high_hab as f64) >= REGION_DOMINANCE_RATIO);

    if let Some(did) = dominant_idx {
        // 只保留 dominant 区域，丢弃其余
        let dominant_pixels = region_pixels.swap_remove(did);

        // 在 dominant 区域内选 CITY_COUNT 个种子（宜居度最高且距离较远）
        let seeds = pick_distant_seeds(&dominant_pixels, &habitability, w);

        // 重新 BFS 划分 dominant 区域
        let mut new_region_id = vec![-1i32; total];

        // 先标记 dominant 内的像素
        let mut dominant_mask = vec![false; total];
        for &p in &dominant_pixels {
            dominant_mask[p] = true;
        }

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

        // 重建 region_pixels
        let mut new_regions: Vec<Vec<usize>> = (0..CITY_COUNT).map(|_| Vec::new()).collect();
        for &p in &dominant_pixels {
            let rid = new_region_id[p];
            if rid >= 0 && (rid as usize) < new_regions.len() {
                new_regions[rid as usize].push(p);
            }
        }
        new_regions.retain(|p| !p.is_empty());
        region_pixels = new_regions;
    }

    // 6. 为每个区域提取多边形轮廓
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

/// 从 dominant 区域内选取 CITY_COUNT 个种子点
///
/// 优先选宜居度最高且彼此远离的像素，确保拆分后各子区域空间合理。
fn pick_distant_seeds(pixels: &[usize], habitability: &[f32], w: usize) -> Vec<usize> {
    if pixels.len() <= CITY_COUNT {
        // 像素数不够，直接取所有
        return pixels.to_vec();
    }

    // 按宜居度排序（降序），取前 20% 作为候选池
    let mut candidates: Vec<usize> = pixels.to_vec();
    candidates.sort_by(|&a, &b| habitability[b].partial_cmp(&habitability[a]).unwrap());
    let pool_size = (candidates.len() / 5).max(CITY_COUNT * 3);
    candidates.truncate(pool_size);

    let mut seeds: Vec<usize> = Vec::new();

    // 取最高的作为第一个种子
    seeds.push(candidates[0]);

    let (w_f64, h_f64) = (MAP_WIDTH as f64, MAP_HEIGHT as f64);
    let max_dist = (w_f64 * w_f64 + h_f64 * h_f64).sqrt();

    while seeds.len() < CITY_COUNT && seeds.len() < candidates.len() {
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

/// 判断两个区域是否相邻（共享边界像素）
fn regions_adjacent(pixels_list: &[Vec<usize>], a: usize, b: usize, w: usize, h: usize) -> bool {
    let pixels_b = &pixels_list[b];
    // 用哈希集快速查找
    use std::collections::HashSet;
    let set_b: HashSet<usize> = pixels_b.iter().copied().collect();

    for &idx in &pixels_list[a] {
        let (cy, cx) = (idx / w, idx % w);
        let neighbors = [
            if cy > 0     { Some((cy - 1) * w + cx) } else { None },
            if cy + 1 < h { Some((cy + 1) * w + cx) } else { None },
            if cx > 0     { Some(cy * w + (cx - 1)) } else { None },
            if cx + 1 < w { Some(cy * w + (cx + 1)) } else { None },
        ];
        for nb in neighbors.iter().flatten() {
            if set_b.contains(nb) {
                return true;
            }
        }
    }
    false
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
