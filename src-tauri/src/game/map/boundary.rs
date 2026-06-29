use std::collections::HashMap;
use noise::{NoiseFn, Perlin};
use super::{MAP_WIDTH, MAP_HEIGHT, GRID_N, WORLD_MIN, WORLD_MAX};
use super::heightmap;

/// 行政边界：多边形 + 像素级掩码
pub(super) struct Boundary {
    /// 世界坐标闭合环 [x0,y0,x1,y1,...]，与 RoadData.points 约定一致
    pub polygon: Vec<f64>,
    /// 512×512 行主序，true = 在边界内
    pub mask: Vec<bool>,
}

impl Boundary {
    /// 查询世界坐标点 (wx, wy) 是否在边界内
    #[allow(dead_code)]
    pub fn contains_world(&self, wx: f64, wy: f64) -> bool {
        let (px, py) = heightmap::world_to_pixel(wx, wy);
        let idx = py as usize * MAP_WIDTH as usize + px as usize;
        self.mask.get(idx).copied().unwrap_or(false)
    }
}

/// 生成一条自然曲折的封闭边界
pub(super) fn generate_boundary(seed: u32) -> Boundary {
    let cell_w = MAP_WIDTH as f64 / GRID_N as f64;   // ≈ 42.67
    let cell_h = MAP_HEIGHT as f64 / GRID_N as f64;

    let mut final_cell_mask: Vec<bool> = Vec::new();
    let mut attempts = 0;

    // 重试循环：覆盖率不达标则换种重试
    while attempts < 8 {
        let current_seed = seed.wrapping_add(attempts as u32);
        let perlin = Perlin::new(current_seed ^ 0xB0D);
        let scale = 0.008 * 0.4;  // MAP_SCALE * 0.4, 低频

        // 1. 在格中心采样 Perlin
        let mut raw = Vec::with_capacity((GRID_N * GRID_N) as usize);
        for j in 0..GRID_N {
            for i in 0..GRID_N {
                let (wx, wy) = cell_center_world(i, j, cell_w, cell_h);
                let nx = wx * scale;
                let ny = wy * scale;
                let mut v = 0.0;
                let mut amp = 1.0;
                let mut freq = 1.0;
                let mut max_amp = 0.0;
                for _ in 0..2 {  // 2 octaves
                    v += perlin.get([nx * freq, ny * freq]) * amp;
                    max_amp += amp;
                    amp *= 0.5;
                    freq *= 2.0;
                }
                raw.push(v / max_amp);
            }
        }

        // 2. 中位值阈值
        let mut sorted = raw.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[raw.len() / 2];
        let cell_mask: Vec<bool> = raw.iter().map(|v| *v >= median).collect();

        // 3. 形态学膨胀 1 次 (8-邻接)
        let dilated = dilate_8(&cell_mask, GRID_N as usize);

        // 4. 取含中心格的最大连通分量 (8-邻接)
        let center_idx = (GRID_N / 2) * GRID_N + (GRID_N / 2);
        let cc = component_containing_center(&dilated, GRID_N as usize, center_idx as usize);

        // 5. 计算覆盖率
        let total = (GRID_N * GRID_N) as usize;
        let count = cc.iter().filter(|&&b| b).count();
        let coverage = count as f64 / total as f64;

        if coverage >= 0.40 && coverage <= 0.95 {
            final_cell_mask = cc;
            break;
        }
        attempts += 1;
    }

    // 回退：距中心半径内全保留
    if final_cell_mask.is_empty() {
        final_cell_mask = fallback_circle(GRID_N as usize);
    }

    // 6. 轮廓提取 → 多边形顶点 (像素坐标)
    let pixel_verts = extract_contour(&final_cell_mask, GRID_N as usize, cell_w, cell_h);

    // 7. 3-tap 移动平均平滑 2 遍
    let smoothed = smooth_polygon(&pixel_verts, 2);

    // 8. 像素 → 世界坐标, 向内缩 1 格, clamp
    let world_poly: Vec<f64> = smoothed
        .iter()
        .map(|&(px, py)| {
            // 向内缩: 从地图边缘缩进约 1 个 cell
            let inset_px = px.clamp(cell_w, MAP_WIDTH as f64 - cell_w);
            let inset_py = py.clamp(cell_h, MAP_HEIGHT as f64 - cell_h);
            heightmap::pixel_to_world(inset_px as u32, inset_py as u32)
        })
        .flat_map(|(wx, wy)| {
            let wx = wx.clamp(WORLD_MIN + 2.0, WORLD_MAX - 2.0);
            let wy = wy.clamp(WORLD_MIN + 2.0, WORLD_MAX - 2.0);
            [wx, wy]
        })
        .collect();

    // 9. 确保顺时针 (有向面积为负则反转)
    let mut final_poly = world_poly.clone();
    if signed_area_2d(&final_poly) < 0.0 {
        reverse_polygon(&mut final_poly);
    }

    // 10. 生成 mask: scanline 填充
    let mask = polygon_to_mask(&final_poly);

    Boundary {
        polygon: final_poly,
        mask,
    }
}

// ── 辅助函数 ──────────────────────────────────────

/// 格中心的世界坐标
fn cell_center_world(i: u32, j: u32, cell_w: f64, cell_h: f64) -> (f64, f64) {
    let px = (i as f64 + 0.5) * cell_w;
    let py = (j as f64 + 0.5) * cell_h;
    heightmap::pixel_to_world(px as u32, py as u32)
}

/// 8-邻接膨胀
fn dilate_8(mask: &[bool], stride: usize) -> Vec<bool> {
    let n = stride;
    let mut out = mask.to_vec();
    for j in 0..n {
        for i in 0..n {
            let idx = j * n + i;
            if mask[idx] {
                // 膨胀到 8 邻域
                for dj in -1..=1 {
                    for di in -1..=1 {
                        if di == 0 && dj == 0 { continue; }
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                            out[nj as usize * n + ni as usize] = true;
                        }
                    }
                }
            }
        }
    }
    out
}

/// 8-邻接 flood fill，取包含指定格(含中心格)的连通分量
fn component_containing_center(mask: &[bool], stride: usize, seed_idx: usize) -> Vec<bool> {
    let n = stride;
    let total = n * n;
    if !mask[seed_idx] {
        return vec![false; total];
    }

    // BFS
    let mut visited = vec![false; total];
    let mut stack = vec![seed_idx];
    visited[seed_idx] = true;
    let mut component: Vec<usize> = Vec::new();

    while let Some(idx) = stack.pop() {
        component.push(idx);
        let i = idx % n;
        let j = idx / n;
        for dj in -1..=1 {
            for di in -1..=1 {
                if di == 0 && dj == 0 { continue; }
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < n as i32 {
                    let nidx = nj as usize * n + ni as usize;
                    if mask[nidx] && !visited[nidx] {
                        visited[nidx] = true;
                        stack.push(nidx);
                    }
                }
            }
        }
    }

    // 只保留这个最大分量，其余 false
    let mut result = vec![false; total];
    for &idx in &component {
        result[idx] = true;
    }
    result
}

/// 回退策略：中心半径内全保留
fn fallback_circle(n: usize) -> Vec<bool> {
    let cx = (n / 2) as f64;
    let cy = cx;
    let radius = n as f64 * 0.35;
    let mut mask = vec![false; n * n];
    for j in 0..n {
        for i in 0..n {
            let dx = i as f64 - cx;
            let dy = j as f64 - cy;
            if dx * dx + dy * dy <= radius * radius {
                mask[j * n + i] = true;
            }
        }
    }
    mask
}

// ── 轮廓提取 ──────────────────────────────────────

/// 从 cell mask 提取多边形轮廓 (像素坐标)
fn extract_contour(mask: &[bool], n: usize, cell_w: f64, cell_h: f64) -> Vec<(f64, f64)> {
    // 收集边界线段: 每个线段连接两个相邻的格点 (cell corners)
    // 格点坐标 (gi, gj) 映射到像素坐标 (gi * cell_w, gj * cell_h)
    //
    // 线段存在于:
    //   - 水平线: 格点 (i,j)-(i+1,j) 之间, 若栅格 [j][i] 与 [j][i-1] 不同… 
    //   更准确: 每条 cell 边, 若相邻两 cell 状态不同则为边界
    
    // 水平边: 在行 j 上, 格点 (i,j)-(i+1,j) 之间
    //   如果 j-1 >= 0: cell (i, j-1) = in, cell (i, j) = out → 上边是边界
    //   如果 j < n: cell (i, j) = in, cell (i, j-1) = out → 下边是边界
    // 其实更简单: 每条水平边上下的 cell 状态不同则为边界
    
    // 简化: 用 HashMap 记录每个格点连接的线段端点
    #[derive(Clone, Copy, Debug)]
    struct EdgeSeg {
        a: (f64, f64),
        b: (f64, f64),
    }

    let mut segments: Vec<EdgeSeg> = Vec::new();

    // 水平边: 在格点 (i,j)-(i+1,j) 之间
    // 上下 cell 为 (i, j-1) 和 (i, j)
    for j in 0..=n {
        for i in 0..n {
            let above = if j > 0 { mask[(j-1) * n + i] } else { false };
            let below = if j < n { mask[j * n + i] } else { false };
            if above != below {
                let x1 = i as f64 * cell_w;
                let y1 = j as f64 * cell_h;
                let x2 = (i + 1) as f64 * cell_w;
                let y2 = j as f64 * cell_h;
                segments.push(EdgeSeg { a: (x1, y1), b: (x2, y2) });
            }
        }
    }

    // 垂直边: 在格点 (i,j)-(i,j+1) 之间
    for i in 0..=n {
        for j in 0..n {
            let left = if i > 0 { mask[j * n + (i-1)] } else { false };
            let right = if i < n { mask[j * n + i] } else { false };
            if left != right {
                let x1 = i as f64 * cell_w;
                let y1 = j as f64 * cell_h;
                let x2 = i as f64 * cell_w;
                let y2 = (j + 1) as f64 * cell_h;
                segments.push(EdgeSeg { a: (x1, y1), b: (x2, y2) });
            }
        }
    }

    if segments.is_empty() {
        return Vec::new();
    }

    // 建立邻接表: 端点→[(线段索引, 另一端点)]
    let mut adj: HashMap<(i64, i64), Vec<(usize, (i64, i64))>> = HashMap::new();
    
    // 将 f64 坐标量化为 i64 格点索引来匹配
    fn key(px: f64, py: f64, cell_w: f64, cell_h: f64) -> (i64, i64) {
        let gi = (px / cell_w).round() as i64;
        let gj = (py / cell_h).round() as i64;
        (gi, gj)
    }

    for (idx, seg) in segments.iter().enumerate() {
        let ka = key(seg.a.0, seg.a.1, cell_w, cell_h);
        let kb = key(seg.b.0, seg.b.1, cell_w, cell_h);
        adj.entry(ka).or_default().push((idx, kb));
        adj.entry(kb).or_default().push((idx, ka));
    }

    // 追踪多边形: 从第一条线段开始
    let mut poly_px: Vec<(f64, f64)> = Vec::new();
    let first = segments[0];
    let start_key = key(first.a.0, first.a.1, cell_w, cell_h);
    let start_dir = dir_from_to(&start_key, &key(first.b.0, first.b.1, cell_w, cell_h));
    let mut curr_key = start_key;
    // 进入当前格点所沿的方向(用于"最右转"选择,保证单一闭合外环不自交)
    let mut entry_dir = dir_from_to(&start_key, &key(first.b.0, first.b.1, cell_w, cell_h));
    // 当回到起点且走了至少一圈时闭合
    let mut visited_segs: std::collections::HashSet<usize> = std::collections::HashSet::new();
    visited_segs.insert(0);

    loop {
        // 当前点的像素坐标
        let px = curr_key.0 as f64 * cell_w;
        let py = curr_key.1 as f64 * cell_h;
        poly_px.push((px, py));

        let neighbors = match adj.get(&curr_key) {
            Some(nb) => nb,
            None => break,
        };

        // 候选: 未走过的线段
        let candidates: Vec<(usize, (i64, i64))> = neighbors.iter()
            .filter(|(idx, _)| !visited_segs.contains(idx))
            .map(|(idx, k)| (*idx, *k))
            .collect();

        if candidates.is_empty() {
            break;
        }

        // 选相对 entry_dir 顺时针最右的边(外轮廓追踪规则)
        let next = candidates.iter().min_by_key(|(_, nk)| {
            let out_dir = dir_from_to(&curr_key, nk);
            turn_clockwise(entry_dir, out_dir)
        }).copied();

        match next {
            Some((idx, next_key)) => {
                visited_segs.insert(idx);
                entry_dir = dir_from_to(&curr_key, &next_key);
                if next_key == start_key && poly_px.len() > 2 && entry_dir == (start_dir) {
                    break; // 回到起点方向一致，闭合
                }
                curr_key = next_key;
            }
            None => break,
        }

        // 安全阀
        if poly_px.len() > 1000 {
            break;
        }
    }

    poly_px
}

/// 从 a 到 b 的格点方向: 0=东 1=南 2=西 3=北(非4连通方向返回-1)
fn dir_from_to(a: &(i64, i64), b: &(i64, i64)) -> i32 {
    let di = b.0 - a.0;
    let dj = b.1 - a.1;
    match (di, dj) {
        (1, 0) => 0,  // 东
        (0, 1) => 1,  // 南(像素 y 向下)
        (-1, 0) => 2, // 西
        (0, -1) => 3, // 北
        _ => -1,
    }
}

/// 从 from_dir 到 to_dir 的顺时针旋转量(0..4)，越大越"右转"
/// 顺时针序: 东(0)→南(1)→西(2)→北(3)→东(0)
fn turn_clockwise(from: i32, to: i32) -> i32 {
    if from < 0 || to < 0 {
        return 99; // 退化: 放到最低优先级
    }
    (to - from).rem_euclid(4)
}

// ── 多边形工具 ────────────────────────────────────

/// 3-tap 移动平均平滑 (窗口 = 3, 迭代多次)
fn smooth_polygon(verts: &[(f64, f64)], passes: usize) -> Vec<(f64, f64)> {
    if verts.len() <= 3 {
        return verts.to_vec();
    }
    let mut pts = verts.to_vec();
    for _ in 0..passes {
        let mut next = pts.clone();
        let len = pts.len();
        for i in 0..len {
            let prev = if i == 0 { len - 1 } else { i - 1 };
            let next_i = if i == len - 1 { 0 } else { i + 1 };
            next[i] = (
                (pts[prev].0 + pts[i].0 + pts[next_i].0) / 3.0,
                (pts[prev].1 + pts[i].1 + pts[next_i].1) / 3.0,
            );
        }
        pts = next;
    }
    pts
}

/// 有向面积 (Shoelace), 正 = 逆时针, 负 = 顺时针
fn signed_area_2d(poly: &[f64]) -> f64 {
    if poly.len() < 6 { return 0.0; }
    let mut area = 0.0;
    let n = poly.len() / 2;
    for i in 0..n {
        let j = (i + 1) % n;
        area += poly[i * 2] * poly[j * 2 + 1];
        area -= poly[j * 2] * poly[i * 2 + 1];
    }
    area * 0.5
}

/// 反转多边形点序
fn reverse_polygon(poly: &mut [f64]) {
    let n = poly.len();
    for i in 0..n / 4 {
        let j = n - 2 - i * 2;
        poly.swap(i * 2, j);
        poly.swap(i * 2 + 1, j + 1);
    }
}

/// 多边形 → 像素掩码 (scanline 填充)
fn polygon_to_mask(poly: &[f64]) -> Vec<bool> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;
    let mut mask = vec![false; total];

    if poly.len() < 6 { return mask; }

    let n = poly.len() / 2;

    // 将世界坐标多边形投影到像素行
    // 对每行 y, 计算多边形与扫描线的交点
    // (世界坐标 → 像素坐标)
    let mut pixel_verts: Vec<(i32, i32)> = Vec::with_capacity(n);
    for i in 0..n {
        let (wx, wy) = (poly[i * 2], poly[i * 2 + 1]);
        let (px, py) = heightmap::world_to_pixel(wx, wy);
        pixel_verts.push((px as i32, py as i32));
    }

    // scanline 填充 (非零环绕规则)
    for scan_y in 0..h as i32 {
        let mut intersections: Vec<i32> = Vec::new();
        for i in 0..n {
            let j = (i + 1) % n;
            let (x1, y1) = pixel_verts[i];
            let (x2, y2) = pixel_verts[j];

            // 忽略水平边
            if y1 == y2 { continue; }

            // 确保扫描线在边的 y 范围内
            if (scan_y < y1.min(y2)) || (scan_y >= y1.max(y2)) { continue; }

            // 计算交点 x
            let t = (scan_y - y1) as f64 / (y2 - y1) as f64;
            let x = x1 as f64 + t * (x2 - x1) as f64;
            intersections.push(x.round() as i32);
        }

        intersections.sort_unstable();

        // 成对填充
        for pair in intersections.chunks(2) {
            if pair.len() < 2 { break; }
            let x_start = pair[0].max(0);
            let x_end = pair[1].min(w as i32 - 1);
            for x in x_start..=x_end {
                let idx = scan_y as usize * w + x as usize;
                if idx < total {
                    mask[idx] = true;
                }
            }
        }
    }

    mask
}
