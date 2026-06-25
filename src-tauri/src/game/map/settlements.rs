use rand::Rng;
use super::{LocationType, MAP_WIDTH, MAP_HEIGHT, LARGE_NAMES, MEDIUM_NAMES, SMALL_NAMES};
use super::heightmap;

/// 仅在高度 h<0.10 的低海拔区域生成位置，不分配类型
pub(super) fn generate_positions(heights: &[f64], seed: u32) -> Vec<(f64, f64)> {
    let mut loc_rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([seed as u8; 32]);

    let max_attempts = 3000;
    let target_count = 120 + (seed as usize % 21); // 120~140

    let mut positions: Vec<(f64, f64)> = Vec::with_capacity(target_count);
    let mut attempts = 0;

    while positions.len() < target_count && attempts < max_attempts {
        attempts += 1;

        let px = loc_rng.gen_range(40..MAP_WIDTH - 40);
        let py = loc_rng.gen_range(40..MAP_HEIGHT - 40);
        let h = heightmap::get_height(heights, px, py);

        if h >= 0.10 {
            continue;
        }

        let (wx, wy) = heightmap::pixel_to_world(px, py);
        let wx = (wx * 10.0).round() / 10.0;
        let wy = (wy * 10.0).round() / 10.0;

        // 检查间距（至少 40 世界单位）
        let too_close = positions.iter().any(|&(x, y)| {
            let dx = x - wx;
            let dy = y - wy;
            (dx * dx + dy * dy) < 1600.0
        });
        if too_close {
            continue;
        }

        positions.push((wx, wy));
    }

    positions
}

/// 从对应规模的名称池中选取一个名字
pub(super) fn pick_name(loc_type: &LocationType, rng: &mut impl Rng) -> String {
    let pool = match loc_type {
        LocationType::Large => LARGE_NAMES,
        LocationType::Medium => MEDIUM_NAMES,
        LocationType::Small => SMALL_NAMES,
    };
    let idx = rng.gen_range(0..pool.len());
    pool[idx].to_string()
}

/// 基于中心地理论的空间约束分配：按分数降序 + 间距约束，城市优先抢占、小镇次之
pub(super) fn classify_by_central_place(
    positions: &[(f64, f64)],
    scores: &[f64],
) -> Vec<usize> {
    let n = positions.len();
    if n == 0 {
        return Vec::new();
    }

    let m = compute_median_nn_distance(positions);
    let (d_l, d_ml, d_m) = (m * 5.0, m * 3.5, m * 2.5);

    let medium_target = (n as f64 * 0.18) as usize;
    let large_cap = 3usize.min(n);

    // 按分数降序排列索引
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| scores[b].partial_cmp(&scores[a]).unwrap());

    let mut result = vec![0usize; n];
    let (mut large, mut medium) = (Vec::new(), Vec::new());

    for &i in &order {
        let (x, y) = positions[i];
        // 城市优先抢占地盘
        if large.len() < large_cap && min_dist_to(x, y, &large, positions) >= d_l {
            result[i] = 2;
            large.push(i);
        }
        // 小镇在剩余空间中选
        else if medium.len() < medium_target
            && min_dist_to(x, y, &large, positions) >= d_ml
            && min_dist_to(x, y, &medium, positions) >= d_m
        {
            result[i] = 1;
            medium.push(i);
        }
    }

    result
}

/// 计算点到一组已选点的最小欧几里得距离
fn min_dist_to(x: f64, y: f64, ids: &[usize], positions: &[(f64, f64)]) -> f64 {
    ids.iter()
        .map(|&j| {
            let dx = x - positions[j].0;
            let dy = y - positions[j].1;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(f64::MAX, f64::min)
}

/// 计算每个点到其最近邻的距离，取中位数（反映点的稀疏程度）
fn compute_median_nn_distance(positions: &[(f64, f64)]) -> f64 {
    let n = positions.len();
    if n <= 1 {
        return 100.0;
    }

    let mut nn_dists = Vec::with_capacity(n);
    for i in 0..n {
        let (xi, yi) = positions[i];
        let mut min_d2 = f64::MAX;
        for j in 0..n {
            if i == j {
                continue;
            }
            let dx = xi - positions[j].0;
            let dy = yi - positions[j].1;
            let d2 = dx * dx + dy * dy;
            if d2 < min_d2 {
                min_d2 = d2;
            }
        }
        nn_dists.push(min_d2.sqrt());
    }

    nn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
    nn_dists[n / 2]
}
