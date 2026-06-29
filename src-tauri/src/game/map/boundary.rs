use noise::{NoiseFn, Perlin};
use super::{MAP_WIDTH, MAP_HEIGHT, WORLD_MIN, WORLD_MAX};
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

// ── 边界采样常量 ──────────────────────────────────

/// 角度采样数（约每 1.6° 一个点）
const ANG_RES: usize = 220;

/// 基础半径占比（相对 ref_side 的一半）
const BASE_RADIUS: f64 = 0.80;

/// 低频大弯曲幅度
const LOW_AMP: f64 = 0.10;

/// 高频细节锯齿幅度
const HI_AMP: f64 = 0.03;

/// 低频空间尺度
const SCALE_LOW: f64 = 3.0;

/// 高频空间尺度
const SCALE_HI: f64 = 16.0;

// ── 主要生成函数 ──────────────────────────────────

/// 生成一条自然曲折的封闭边界（径向偏移曲线算法）
pub(super) fn generate_boundary(seed: u32) -> Boundary {
    let perlin = Perlin::new(seed ^ 0xB0D);
    let perlin_hi = Perlin::new(seed.wrapping_add(0x1F3));

    let ref_side = (WORLD_MAX - WORLD_MIN).abs(); // 1600
    let center = (WORLD_MIN + WORLD_MAX) / 2.0;   // 0
    let r_base = ref_side * 0.5 * BASE_RADIUS;

    let mut points: Vec<(f64, f64)> = Vec::with_capacity(ANG_RES);

    for i in 0..ANG_RES {
        let theta = (i as f64 / ANG_RES as f64) * std::f64::consts::TAU;
        let ux = theta.cos();
        let uy = theta.sin();

        // 多频段 Perlin 噪声调制半径
        let nx = ux * SCALE_LOW;
        let ny = uy * SCALE_LOW;

        let low_noise = perlin.get([nx, ny]);
        let low = low_noise.signum() * low_noise.abs().sqrt();

        let hi_nx = ux * SCALE_HI + 100.0; // 相位偏移，与低频解耦
        let hi_ny = uy * SCALE_HI + 100.0;
        let hi = perlin_hi.get([hi_nx, hi_ny]);

        let radius = r_base * (1.0 + LOW_AMP * low + HI_AMP * hi);

        // 防御性 clamp
        let radius = radius.clamp(r_base * 0.45, ref_side * 0.495);

        let wx = center + ux * radius;
        let wy = center + uy * radius;

        // 世界坐标 clamp
        let wx = wx.clamp(WORLD_MIN + 2.0, WORLD_MAX - 2.0);
        let wy = wy.clamp(WORLD_MIN + 2.0, WORLD_MAX - 2.0);

        points.push((wx, wy));
    }

    // 轻度平滑：3-tap 移动平均 1 遍（仅抹掉角度采样的高频毛刺）
    let smoothed = smooth_ring(&points, 1);

    // 展平为 Vec<f64>
    let mut final_poly: Vec<f64> = Vec::with_capacity(ANG_RES * 2);
    for &(x, y) in &smoothed {
        final_poly.push(x);
        final_poly.push(y);
    }

    // 确保顺时针（有向面积为负则反转）
    if signed_area_2d(&final_poly) < 0.0 {
        reverse_polygon(&mut final_poly);
    }

    // 生成 mask：scanline 填充
    let mask = polygon_to_mask(&final_poly);

    Boundary {
        polygon: final_poly,
        mask,
    }
}

// ── 环状平滑 ──────────────────────────────────────

/// 3-tap 移动平均平滑（窗口 = 3，仅抹掉高频毛刺，保留弯曲）
fn smooth_ring(verts: &[(f64, f64)], passes: usize) -> Vec<(f64, f64)> {
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

// ── 多边形工具 ────────────────────────────────────

/// 有向面积（Shoelace），正 = 逆时针，负 = 顺时针
fn signed_area_2d(poly: &[f64]) -> f64 {
    if poly.len() < 6 {
        return 0.0;
    }
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

/// 多边形 → 像素掩码（scanline 填充，非零环绕规则）
fn polygon_to_mask(poly: &[f64]) -> Vec<bool> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;
    let total = w * h;
    let mut mask = vec![false; total];

    if poly.len() < 6 {
        return mask;
    }

    let n = poly.len() / 2;

    // 将世界坐标多边形投影到像素行
    let mut pixel_verts: Vec<(i32, i32)> = Vec::with_capacity(n);
    for i in 0..n {
        let (wx, wy) = (poly[i * 2], poly[i * 2 + 1]);
        let (px, py) = heightmap::world_to_pixel(wx, wy);
        pixel_verts.push((px as i32, py as i32));
    }

    // scanline 填充（非零环绕规则）
    for scan_y in 0..h as i32 {
        let mut intersections: Vec<i32> = Vec::new();
        for i in 0..n {
            let j = (i + 1) % n;
            let (x1, y1) = pixel_verts[i];
            let (x2, y2) = pixel_verts[j];

            // 忽略水平边
            if y1 == y2 {
                continue;
            }

            // 确保扫描线在边的 y 范围内
            if (scan_y < y1.min(y2)) || (scan_y >= y1.max(y2)) {
                continue;
            }

            // 计算交点 x
            let t = (scan_y - y1) as f64 / (y2 - y1) as f64;
            let x = x1 as f64 + t * (x2 - x1) as f64;
            intersections.push(x.round() as i32);
        }

        intersections.sort_unstable();

        // 成对填充
        for pair in intersections.chunks(2) {
            if pair.len() < 2 {
                break;
            }
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
