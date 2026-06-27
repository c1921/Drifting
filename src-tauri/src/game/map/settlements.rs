use rand::Rng;
use super::{LocationType, MAP_WIDTH, MAP_HEIGHT, WORLD_MIN, WORLD_MAX, LARGE_NAMES, MEDIUM_NAMES, SMALL_NAMES};
use super::heightmap;
use super::boundary::Boundary;
use super::{CITY_COUNT, TOWN_COUNT, TOTAL_TARGET,
    DIST_CITY_CITY, DIST_CITY_TOWN, DIST_TOWN_TOWN,
    DIST_TOWN_VILLAGE, DIST_VILLAGE_VILLAGE, DIST_VILLAGE_CITY,
    ATTRACT_NEAR_PROB, ATTRACT_CITY_TOWN_FALLOFF,
    ATTRACT_TOWN_VILLAGE_FALLOFF, ATTRACT_CITY_VILLAGE_FALLOFF};

/// 宜居度最低接受阈值: 低于此值的像素不被选作采样候选
const MIN_HAB: f32 = 0.05;

/// 宜居度加权 CDF (边界内 + 高于最低宜居度)。在 generate_map 构建一次, 供三层采样复用。
pub(super) struct HabCdf {
    pixels: Vec<u32>,    // 候选像素索引 (py * MAP_WIDTH + px)
    weights: Vec<f32>,   // 累积权重, weights[i] = sum(0..=i)
    total: f32,
}

/// 构建宜居度加权 CDF
pub(super) fn build_hab_cdf(boundary: &Boundary, habitability: &[f32]) -> HabCdf {
    let mut pixels: Vec<u32> = Vec::new();
    let mut weights: Vec<f32> = Vec::new();
    let mut total = 0.0f32;
    for py in 0..MAP_HEIGHT {
        for px in 0..MAP_WIDTH {
            let idx = (py * MAP_WIDTH + px) as usize;
            if boundary.mask[idx] && habitability[idx] > MIN_HAB {
                total += habitability[idx];
                pixels.push(py * MAP_WIDTH + px);
                weights.push(total);
            }
        }
    }
    HabCdf { pixels, weights, total }
}

/// 通用地点放置：宜居度加权采样 + 吸引力驱动 + 边界约束
fn place_settlements(
    _heights: &[f64],
    boundary: &Boundary,
    habitability: &[f32],
    cdf: &HabCdf,
    layer_seed: u32,
    target_count: usize,
    constraint_groups: &[(&[(f64, f64)], f64)],   // (已有位置, 最小距离)
    dist_to_peer: f64,                              // 同层最小间距
    attractors: &[(&[(f64, f64)], f64)],            // (吸引子中心, falloff)
    max_attempts: usize,
) -> Vec<(f64, f64)> {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([layer_seed as u8; 32]);
    let mut placed: Vec<(f64, f64)> = Vec::with_capacity(target_count);
    let mut attempts = 0;

    let sq_peer = dist_to_peer * dist_to_peer;
    let sq_constraints: Vec<(&[(f64, f64)], f64)> = constraint_groups
        .iter()
        .map(|(group, d)| (*group, d * d))
        .collect();

    // 展平所有吸引子（每个中心独立，继承其分组 falloff）
    let flat_attractors: Vec<((f64, f64), f64)> = attractors
        .iter()
        .flat_map(|(centers, falloff)| centers.iter().map(move |&c| (c, *falloff)))
        .collect();

    while placed.len() < target_count && attempts < max_attempts {
        attempts += 1;

        // ── 采样候选坐标 ──────────────────────────
        let (wx, wy) = if !flat_attractors.is_empty() && rng.gen_bool(ATTRACT_NEAR_PROB) {
            // 吸引力路径：在吸引子 falloff 半径内以面积均匀分布采样
            let idx = rng.gen_range(0..flat_attractors.len());
            let ((cx, cy), falloff) = flat_attractors[idx];
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let u: f64 = rng.gen_range(0.0..1.0);
            let r = falloff * u * u;
            let sx = cx + r * angle.cos();
            let sy = cy + r * angle.sin();

            // 边界硬过滤：吸引力路径可能越界
            if !boundary.contains_world(sx, sy) {
                continue;
            }
            (sx, sy)
        } else if !cdf.pixels.is_empty() && cdf.total > 0.0 {
            // 宜居度加权采样路径：按 CDF 选像素
            let r = rng.gen::<f32>() * cdf.total;
            let idx = match cdf.weights.binary_search_by(|&w| w.partial_cmp(&r).unwrap()) {
                Ok(i) => i,
                Err(i) => i.min(cdf.weights.len() - 1),
            };
            let pixel = cdf.pixels[idx];
            let px = pixel % MAP_WIDTH;
            let py = pixel / MAP_WIDTH;
            let (sx, sy) = heightmap::pixel_to_world(px, py);
            (sx, sy)
        } else {
            // 兜底：均匀像素采样（极少发生）
            let px = rng.gen_range(0..MAP_WIDTH);
            let py = rng.gen_range(0..MAP_HEIGHT);
            let (sx, sy) = heightmap::pixel_to_world(px, py);
            (sx, sy)
        };

        // ── 世界坐标边界检查 ────────────────────────
        if wx < WORLD_MIN || wx > WORLD_MAX || wy < WORLD_MIN || wy > WORLD_MAX {
            continue;
        }

        let wx = (wx * 10.0).round() / 10.0;
        let wy = (wy * 10.0).round() / 10.0;

        // ── 宜居度软接受 ──────────────────────────
        let (px, py) = heightmap::world_to_pixel(wx, wy);
        let idx = (py * MAP_WIDTH + px) as usize;
        let hab = habitability.get(idx).copied().unwrap_or(0.0) as f64;
        if rng.gen::<f64>() > hab {
            continue;
        }

        // ── 最小距离约束（上层分组）────────────────
        let mut skip = false;
        for (group, sq_d) in &sq_constraints {
            if group.iter().any(|&(x, y)| {
                let dx = x - wx;
                let dy = y - wy;
                dx * dx + dy * dy < *sq_d
            }) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        // ── 同层间距 ──────────────────────────────
        if placed.iter().any(|&(x, y)| {
            let dx = x - wx;
            let dy = y - wy;
            dx * dx + dy * dy < sq_peer
        }) {
            continue;
        }

        placed.push((wx, wy));
    }

    placed
}

/// 生成城市：宜居度加权分布，仅城市间最小距离约束
pub(super) fn generate_cities(
    heights: &[f64],
    boundary: &Boundary,
    habitability: &[f32],
    cdf: &HabCdf,
    seed: u32,
) -> Vec<(f64, f64)> {
    let cities = place_settlements(
        heights, boundary, habitability, cdf, seed, CITY_COUNT,
        &[],
        DIST_CITY_CITY,
        &[],
        3000,
    );
    if cities.len() < CITY_COUNT {
        log::warn!(
            "Map [seed={}]: only placed {}/{} cities",
            seed, cities.len(), CITY_COUNT
        );
    }
    cities
}

/// 生成小镇：由城市吸引，满足城市-小镇最小距离 + 镇间最小距离
pub(super) fn generate_towns(
    heights: &[f64],
    boundary: &Boundary,
    habitability: &[f32],
    cdf: &HabCdf,
    seed: u32,
    cities: &[(f64, f64)],
) -> Vec<(f64, f64)> {
    let towns = place_settlements(
        heights, boundary, habitability, cdf,
        seed.wrapping_add(1),
        TOWN_COUNT,
        &[(cities, DIST_CITY_TOWN)],
        DIST_TOWN_TOWN,
        &[(cities, ATTRACT_CITY_TOWN_FALLOFF)],
        3000,
    );
    if towns.len() < TOWN_COUNT {
        log::warn!(
            "Map [seed={}]: only placed {}/{} towns",
            seed, towns.len(), TOWN_COUNT
        );
    }
    towns
}

/// 生成村庄：由城市(弱)和小镇(强)吸引，满足对城市/小镇的最小距离 + 村间最小距离
pub(super) fn generate_villages(
    heights: &[f64],
    boundary: &Boundary,
    habitability: &[f32],
    cdf: &HabCdf,
    seed: u32,
    cities: &[(f64, f64)],
    towns: &[(f64, f64)],
) -> Vec<(f64, f64)> {
    let village_target = TOTAL_TARGET.saturating_sub(CITY_COUNT + TOWN_COUNT);
    let villages = place_settlements(
        heights, boundary, habitability, cdf,
        seed.wrapping_add(2),
        village_target,
        &[(cities, DIST_VILLAGE_CITY), (towns, DIST_TOWN_VILLAGE)],
        DIST_VILLAGE_VILLAGE,
        &[
            (cities, ATTRACT_CITY_VILLAGE_FALLOFF),
            (towns, ATTRACT_TOWN_VILLAGE_FALLOFF),
        ],
        5000,
    );
    if villages.len() < village_target {
        log::warn!(
            "Map [seed={}]: only placed {}/{} villages",
            seed, villages.len(), village_target
        );
    }
    villages
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
