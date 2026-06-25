use std::collections::BinaryHeap;
use super::{LocationData, RoadData, MAP_WIDTH, MAP_HEIGHT, SLOPE_PENALTY};
use super::heightmap;

/// 基于边集直接生成道路，所有边同等对待，不做分层
pub(super) fn generate_roads_from_edges(
    locations: &[LocationData],
    edges: &[(usize, usize)],
    heights: &[f64],
) -> Vec<RoadData> {
    let mut roads = Vec::with_capacity(edges.len());

    for &(i, j) in edges {
        let road = create_road_segment(
            locations[i].x, locations[i].y,
            locations[j].x, locations[j].y,
            heights,
        );
        roads.push(road);
    }

    roads
}

// ── A* 寻路 ──────────────────────────────────────

#[derive(Clone)]
struct AStarNode {
    f_score: f64,
    pos: (u32, u32),
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star_path(
    heights: &[f64],
    start: (u32, u32),
    goal: (u32, u32),
) -> Option<Vec<(u32, u32)>> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;

    let mut g = vec![f64::MAX; w * h];
    let mut parent = vec![(u32::MAX, u32::MAX); w * h];
    let mut open = BinaryHeap::new();

    let h_start = ((start.0 as f64 - goal.0 as f64).powi(2)
        + (start.1 as f64 - goal.1 as f64).powi(2))
    .sqrt();
    g[(start.1 as usize) * w + start.0 as usize] = 0.0;
    open.push(AStarNode { f_score: h_start, pos: start });

    const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    const BASE_COST: [f64; 8] = [1.414, 1.0, 1.414, 1.0, 1.0, 1.414, 1.0, 1.414];

    let goal_f = (goal.0 as f64, goal.1 as f64);

    while let Some(node) = open.pop() {
        if node.pos == goal {
            let mut path = Vec::new();
            let mut cur = node.pos;
            while cur != start {
                path.push(cur);
                let idx = cur.1 as usize * w + cur.0 as usize;
                cur = parent[idx];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let cur_idx = node.pos.1 as usize * w + node.pos.0 as usize;
        let cur_g = g[cur_idx];
        let cur_h = heights[cur_idx];

        for i in 0..8 {
            let nx = node.pos.0 as i32 + DX[i];
            let ny = node.pos.1 as i32 + DY[i];

            if nx < 0 || nx >= MAP_WIDTH as i32 || ny < 0 || ny >= MAP_HEIGHT as i32 {
                continue;
            }

            let nu = nx as u32;
            let nv = ny as u32;
            let nidx = nv as usize * w + nu as usize;
            let nh = heights[nidx];

            let dh = (nh - cur_h).abs();
            let step_cost = BASE_COST[i] + dh * SLOPE_PENALTY;
            let tentative_g = cur_g + step_cost;

            if tentative_g < g[nidx] {
                g[nidx] = tentative_g;
                parent[nidx] = node.pos;

                let heuristic = ((nu as f64 - goal_f.0).powi(2)
                    + (nv as f64 - goal_f.1).powi(2))
                .sqrt();
                open.push(AStarNode { f_score: tentative_g + heuristic, pos: (nu, nv) });
            }
        }
    }

    None
}

// ── RDP 路径简化 ──────────────────────────────────

fn perp_distance(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;
    let dx = bx - ax;
    let dy = by - ay;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-10 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }

    let t = ((px - ax) * dx + (py - ay) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let cx = ax + t * dx;
    let cy = ay + t * dy;
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

fn rdp_simplify(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut max_d = 0.0;
    let mut max_i = 0;
    let first = points[0];
    let last = points[points.len() - 1];

    for i in 1..points.len() - 1 {
        let d = perp_distance(points[i], first, last);
        if d > max_d {
            max_d = d;
            max_i = i;
        }
    }

    if max_d > epsilon {
        let mut left = rdp_simplify(&points[..=max_i], epsilon);
        let right = rdp_simplify(&points[max_i..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![points[0], points[points.len() - 1]]
    }
}

// ── 创建单条道路 ──────────────────────────────────

fn create_road_segment(
    from_x: f64, from_y: f64,
    to_x: f64, to_y: f64,
    heights: &[f64],
) -> RoadData {
    let start_px = heightmap::world_to_pixel(from_x, from_y);
    let end_px = heightmap::world_to_pixel(to_x, to_y);

    if let Some(pixel_path) = a_star_path(heights, start_px, end_px) {
        let world_path: Vec<(f64, f64)> = pixel_path
            .iter()
            .map(|&(px, py)| heightmap::pixel_to_world(px, py))
            .collect();

        let simplified = rdp_simplify(&world_path, 3.0);

        let mut points = Vec::with_capacity(simplified.len() * 2 + 4);
        points.push((from_x * 10.0).round() / 10.0);
        points.push((from_y * 10.0).round() / 10.0);
        for &(x, y) in &simplified[1..simplified.len().saturating_sub(1)] {
            points.push((x * 10.0).round() / 10.0);
            points.push((y * 10.0).round() / 10.0);
        }
        points.push((to_x * 10.0).round() / 10.0);
        points.push((to_y * 10.0).round() / 10.0);

        RoadData { points }
    } else {
        let mid_x = (from_x + to_x) / 2.0;
        let mid_y = (from_y + to_y) / 2.0;
        let dx = to_x - from_x;
        let dy = to_y - from_y;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let perp_x = -dy / len;
        let perp_y = dx / len;
        let offset = len * 0.15;
        let cp1x = mid_x + perp_x * offset * 0.8;
        let cp1y = mid_y + perp_y * offset * 0.8;
        let cp2x = mid_x - perp_x * offset * 0.5;
        let cp2y = mid_y - perp_y * offset * 0.5;

        RoadData {
            points: vec![
                (from_x * 10.0).round() / 10.0,
                (from_y * 10.0).round() / 10.0,
                (cp1x * 10.0).round() / 10.0,
                (cp1y * 10.0).round() / 10.0,
                (cp2x * 10.0).round() / 10.0,
                (cp2y * 10.0).round() / 10.0,
                (to_x * 10.0).round() / 10.0,
                (to_y * 10.0).round() / 10.0,
            ],
        }
    }
}
