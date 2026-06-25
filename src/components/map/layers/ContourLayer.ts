import { Container, Graphics } from "pixi.js"
import type { ThemeColors } from "../utils/themeUtils"
import type { ContourData } from "@/map/contourExtractor"

/** 地图世界坐标边界（与后端和等高线提取器一致） */
const WORLD_MIN = -800
const WORLD_MAX = 800

/** 四角 [左上, 右上, 右下, 左下] */
const CORNERS: [number, number][] = [
  [WORLD_MIN, WORLD_MAX],
  [WORLD_MAX, WORLD_MAX],
  [WORLD_MAX, WORLD_MIN],
  [WORLD_MIN, WORLD_MIN],
]

export function createContourLayer(
  contours: ContourData[],
  colors: ThemeColors,
): Container {
  const layer = new Container()

  for (let i = 0; i < contours.length; i++) {
    const c = contours[i]
    const pts = c.points
    if (pts.length < 4) continue

    const g = new Graphics()

    // 用直线段绘制等高线（不二次平滑以避免扭曲）
    g.moveTo(pts[0], pts[1])
    for (let j = 2; j < pts.length - 1; j += 2) {
      g.lineTo(pts[j], pts[j + 1])
    }

    if (!c.closed) {
      // 不闭合的等高线：沿地图边界从终点走回起点（90° 折线）
      traceBoundaryClosure(g, pts)
    }
    g.closePath()

    // 填充 + 描边（基于实际高度 level 而非数组索引）
    const t = (c.level - 0.2) / 0.6  // 将 level 0.2~0.8 归一化到 0~1
    const alpha = 0.04 + t * 0.06
    g.fill({ color: colors.mutedForeground, alpha })
    g.stroke({
      width: 0.5 + t * 0.8,
      color: colors.mutedForeground,
      alpha: 0.15 + t * 0.15,
    })

    layer.addChild(g)
  }

  return layer
}

/**
 * 沿地图边界将不闭合等高线的终点走回起点（90° 折线，不切穿地图）
 * 选择经过角点更少（≈ 更短）的那条边界路径。
 */
function traceBoundaryClosure(g: Graphics, pts: number[]) {
  const sx = pts[0], sy = pts[1]
  const ex = pts[pts.length - 2], ey = pts[pts.length - 1]

  const eIdx = edgeIndex(ex, ey)
  const sIdx = edgeIndex(sx, sy)

  if (eIdx === sIdx) {
    // 同一条边上：直线连回即可（不会切穿地图，因为贴边）
    return // 由外层的 closePath() 完成
  }

  // 两条可能的边界路径（顺时针方向经过的角点 vs 逆时针）
  const cw: number[] = []   // eIdx → … → sIdx  正向走
  const ccw: number[] = []  // 反向走

  let i = eIdx
  while (i !== sIdx) {
    i = (i + 1) % 4
    cw.push(i)
  }

  i = eIdx
  while (true) {
    ccw.push(i)
    i = (i + 3) % 4
    if (i === sIdx) break
  }

  // 选角点更少的路径
  const path = cw.length <= ccw.length ? cw : ccw
  for (const ci of path) {
    g.lineTo(CORNERS[ci][0], CORNERS[ci][1])
  }
}

/** 返回点 (x,y) 所在的边界边索引 0=上 1=右 2=下 3=左 */
function edgeIndex(x: number, y: number): number {
  const eps = 0.1
  if (Math.abs(y - WORLD_MAX) < eps) return 0
  if (Math.abs(x - WORLD_MAX) < eps) return 1
  if (Math.abs(y - WORLD_MIN) < eps) return 2
  if (Math.abs(x - WORLD_MIN) < eps) return 3
  // 极小偏移时找最近边
  const ds = [Math.abs(y - WORLD_MAX), Math.abs(x - WORLD_MAX),
              Math.abs(y - WORLD_MIN), Math.abs(x - WORLD_MIN)]
  return ds.indexOf(Math.min(...ds))
}
