use serde::Serialize;

/// 单条等高线数据（平坦的浮点数组，便于序列化）
#[derive(Debug, Clone, Serialize)]
pub struct ContourData {
    pub points: Vec<f64>, // 扁平 [x, y, x, y, ...] 世界坐标
    pub closed: bool,     // 是否闭合
    pub level: f32,       // 对应高度阈值 (0~1)
}

/// 从高度网格计算等高线（基于 contour crate 的 Marching Squares）
pub fn compute_contours(heightmap: &[f64]) -> Vec<ContourData> {
    let width = super::MAP_WIDTH as usize;
    let height = super::MAP_HEIGHT as usize;
    if width == 0 || height == 0 || heightmap.len() < width * height {
        return Vec::new();
    }

    let levels: [f64; 8] = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

    // 世界坐标映射：grid(0,0) → world(-800,-800), grid(511,511) → world(800,800)
    // 与 HeightmapLayer.ts 的 wy = WORLD_MIN + gj * cellH 保持一致
    let x_step = (super::WORLD_MAX - super::WORLD_MIN) / (width - 1) as f64;
    let y_step = (super::WORLD_MAX - super::WORLD_MIN) / (height - 1) as f64;

    let builder = contour::ContourBuilder::new(width, height, false)
        .x_step(x_step)
        .y_step(y_step)
        .x_origin(super::WORLD_MIN)
        .y_origin(super::WORLD_MIN);

    // heightmap 是 Vec<f64>，直接传入
    let lines = match builder.lines(heightmap, &levels) {
        Ok(l) => l,
        Err(e) => {
            log::error!("Failed to compute contours: {}", e);
            return Vec::new();
        }
    };

    let mut result = Vec::new();

    for line in lines {
        let threshold = line.threshold();
        let (multi_line_string, _) = line.into_inner();

        for line_string in multi_line_string.0 {
            if line_string.0.len() < 3 {
                continue; // 至少 3 个点才构成可用折线
            }

            let points: Vec<f64> = line_string
                .0
                .iter()
                .flat_map(|coord| vec![coord.x, coord.y])
                .collect();

            // 判断是否闭合：首尾距离 < 1 世界单位
            let first = &line_string.0[0];
            let last = line_string.0.last().unwrap();
            let dx = first.x - last.x;
            let dy = first.y - last.y;
            let closed = (dx * dx + dy * dy) < 1.0;

            if points.len() >= 6 {
                result.push(ContourData {
                    points,
                    closed,
                    level: threshold as f32,
                });
            }
        }
    }

    result
}
