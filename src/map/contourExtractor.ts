/**
 * Marching Squares 等高线提取器
 * 从高度网格中提取等值线折线
 */

export interface ContourData {
  points: number[] // flattened [x, y, x, y, ...]
  closed: boolean  // 是否为闭合环
  level: number    // 该等高线对应的高度阈值 (0~1)
}

/** 网格采样参数：控制每多少个像素采一个样点 */
const SAMPLE_STEP = 3

/**
 * 从高度网格提取等高线
 * @param heightmap - 高度值 0~1 的扁平数组（行主序）
 * @param width  - 网格宽度（像素）
 * @param height - 网格高度（像素）
 * @param levels - 等高线阈值数组（0~1），默认 7 层
 * @returns 每层等高线的折线点数组
 */
export function extractContours(
  heightmap: Float32Array | number[],
  width: number,
  height: number,
  levels: number[] = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
): ContourData[] {
  const hArr = heightmap instanceof Float32Array ? heightmap : new Float32Array(heightmap)
  const result: ContourData[] = []

  const sW = Math.floor(width / SAMPLE_STEP)
  const sH = Math.floor(height / SAMPLE_STEP)

  // 创建采样后的网格
  const grid = new Float32Array(sW * sH)
  for (let gy = 0; gy < sH; gy++) {
    for (let gx = 0; gx < sW; gx++) {
      const px = gx * SAMPLE_STEP
      const py = gy * SAMPLE_STEP
      const idx = py * width + px
      grid[gy * sW + gx] = idx < hArr.length ? hArr[idx] : 0
    }
  }

  // 世界坐标范围映射
  const worldMin = -800
  const worldMax = 800

  function gridToWorld(gx: number, gy: number): [number, number] {
    return [
      (gx / (sW - 1)) * (worldMax - worldMin) + worldMin,
      (gy / (sH - 1)) * (worldMax - worldMin) + worldMin,
    ]
  }

  for (const level of levels) {
    const segments: [number, number, number, number][] = []

    for (let gy = 0; gy < sH - 1; gy++) {
      for (let gx = 0; gx < sW - 1; gx++) {
        const v00 = grid[gy * sW + gx]         // top-left
        const v10 = grid[gy * sW + gx + 1]     // top-right
        const v01 = grid[(gy + 1) * sW + gx]   // bottom-left
        const v11 = grid[(gy + 1) * sW + gx + 1] // bottom-right

        const b00 = v00 >= level ? 1 : 0
        const b10 = v10 >= level ? 1 : 0
        const b01 = v01 >= level ? 1 : 0
        const b11 = v11 >= level ? 1 : 0
        const code = (b00 << 3) | (b10 << 2) | (b01 << 1) | b11

        if (code === 0 || code === 15) continue

        const [x0, y0] = gridToWorld(gx, gy)       // top-left
        const [x1, y1] = gridToWorld(gx + 1, gy)   // top-right
        const [x2, y2] = gridToWorld(gx, gy + 1)   // bottom-left
        const [x3, y3] = gridToWorld(gx + 1, gy + 1) // bottom-right

        // 四个边上的插值点
        const top = lerp2d(x0, y0, v00, x1, y1, v10, level)
        const right = lerp2d(x1, y1, v10, x3, y3, v11, level)
        const bottom = lerp2d(x2, y2, v01, x3, y3, v11, level)
        const left = lerp2d(x0, y0, v00, x2, y2, v01, level)

        // 中心点值用于 saddle 消歧
        const center = (v00 + v10 + v01 + v11) / 4

        switch (code) {
          // ##########################################
          // 位编码: code = (TL << 3) | (TR << 2) | (BL << 1) | BR
          // 角点: TL=top-left, TR=top-right, BL=bottom-left, BR=bottom-right
          // 边: top(top), right(right), bottom(bottom), left(left)
          // ##########################################
          //
          // 单角 — 只有 1 个角在等高线内
          // 0001: BR → bottom, right
          case 1:  case 14: pushSeg(segments, bottom, right); break
          // 0010: BL → left, bottom
          case 2:  case 13: pushSeg(segments, left, bottom); break
          // 0100: TR → top, right
          case 4:  case 11: pushSeg(segments, top, right); break
          // 1000: TL → left, top
          case 7:  case 8:  pushSeg(segments, left, top); break
          //
          // 两角相邻（在同一条边上）
          // 0011: BL+BR → left, right
          case 3:  case 12: pushSeg(segments, left, right); break
          // 0101: TR+BR → top, bottom（非鞍点，因 TR 和 BR 共享同一边 right）
          case 5:           pushSeg(segments, top, bottom); break
          // 1010: TL+BL → top, bottom
          case 10:          pushSeg(segments, top, bottom); break
          //
          // 鞍点（对角两角在等高线内，四条边都有交点）
          // 0110: TR+BL
          case 6: {
            if (center >= level) {
              // 中心值高 → 隔离 TR↔BL 方向
              pushSeg(segments, top, right)
              pushSeg(segments, bottom, left)
            } else {
              // 中心值低 → 隔离 TL↔BR 方向
              pushSeg(segments, top, left)
              pushSeg(segments, bottom, right)
            }
            break
          }
          // 1001: TL+BR
          case 9: {
            if (center >= level) {
              pushSeg(segments, top, left)
              pushSeg(segments, bottom, right)
            } else {
              pushSeg(segments, top, right)
              pushSeg(segments, bottom, left)
            }
            break
          }
        }
      }
    }

    // 将线段连接成连续折线
    const polylines = connectSegments(segments)
    for (const pl of polylines) {
      if (pl.points.length >= 6) {
        pl.level = level
        result.push(pl)
      }
    }
  }

  return result
}

// ── 辅助函数 ──────────────────────────────────────

/** 两点间线性插值，找出等高线与边的交点 */
function lerp2d(
  xa: number, ya: number, va: number,
  xb: number, yb: number, vb: number,
  level: number,
): [number, number] {
  if (Math.abs(vb - va) < 1e-10) return [(xa + xb) / 2, (ya + yb) / 2]
  const t = (level - va) / (vb - va)
  return [xa + (xb - xa) * t, ya + (yb - ya) * t]
}

/** 添加线段 */
function pushSeg(
  segments: [number, number, number, number][],
  a: [number, number],
  b: [number, number],
) {
  segments.push([a[0], a[1], b[0], b[1]])
}

/**
 * 将散乱线段连接成连续折线
 * 每条线段是 (x1,y1,x2,y2) 的四元组，方向随意
 */
function connectSegments(segments: [number, number, number, number][]): ContourData[] {
  if (segments.length === 0) return []

  // 构建邻接表：每个端点坐标 -> 连接的线段列表
  const adj = new Map<string, { idx: number; side: 'a' | 'b' }[]>()
  const key = (x: number, y: number) => `${Math.round(x * 100)},${Math.round(y * 100)}`

  for (let i = 0; i < segments.length; i++) {
    const s = segments[i]
    const ka = key(s[0], s[1])
    const kb = key(s[2], s[3])

    if (!adj.has(ka)) adj.set(ka, [])
    if (!adj.has(kb)) adj.set(kb, [])

    adj.get(ka)!.push({ idx: i, side: 'a' })
    adj.get(kb)!.push({ idx: i, side: 'b' })
  }

  const used = new Set<number>()
  const result: ContourData[] = []

  for (let i = 0; i < segments.length; i++) {
    if (used.has(i)) continue

    // 当前折线点序列
    const pts: [number, number][] = []

    // 以该线段为起点
    let seg = segments[i]
    used.add(i)


    // 从端点开始构建
    // 先把初始线段两个端点都加入
    pts.push([seg[0], seg[1]])
    pts.push([seg[2], seg[3]])

    // 从尾部延伸
    let tail = [seg[2], seg[3]] as [number, number]
    let extended = true
    while (extended) {
      extended = false
      const tk = key(tail[0], tail[1])
      const neighbors = adj.get(tk) ?? []
      for (const nb of neighbors) {
        if (used.has(nb.idx)) continue
        used.add(nb.idx)
        const ns = segments[nb.idx]
        if (nb.side === 'a') {
          // ns 的 a 端是尾巴，继续向 b 端延伸
          pts.push([ns[2], ns[3]])
          tail = [ns[2], ns[3]]
        } else {
          // ns 的 b 端是尾巴，继续向 a 端延伸
          pts.push([ns[0], ns[1]])
          tail = [ns[0], ns[1]]
        }
        extended = true
        break
      }
    }

    // 从头部反向延伸
    let head = [seg[0], seg[1]] as [number, number]
    extended = true
    while (extended) {
      extended = false
      const hk = key(head[0], head[1])
      const neighbors = adj.get(hk) ?? []
      for (const nb of neighbors) {
        if (used.has(nb.idx)) continue
        used.add(nb.idx)
        const ns = segments[nb.idx]
        if (nb.side === 'a') {
          // ns 的 a 端是头部，ns 的 b 端变成新头部，插入在最前面
          pts.unshift([ns[2], ns[3]])
          head = [ns[2], ns[3]]
        } else {
          pts.unshift([ns[0], ns[1]])
          head = [ns[0], ns[1]]
        }
        extended = true
        break
      }
    }

    // 判断是否闭合：首尾端点是否足够接近
    const first = pts[0]
    const last = pts[pts.length - 1]
    const dx = first[0] - last[0]
    const dy = first[1] - last[1]
    const closed = (dx * dx + dy * dy) < 1.0

    // 展开为 flat 数组
    const flat: number[] = []
    for (const p of pts) {
      flat.push(p[0], p[1])
    }

    result.push({ points: flat, closed, level: 0 })
  }

  return result
}
