import { Container, Graphics } from "pixi.js"

/**
 * 宜居度热力图图层
 *
 * 将 512×512 宜居度数组下采样到 128×128，逐格绘制半透明矩形。
 * 颜色映射：0.0 = 透明/暗红, 0.5 = 黄, 1.0 = 绿。
 * 置于等高线层下方。
 */
export function createHabitabilityLayer(
  habitability: number[],
  width: number,
  height: number,
): Container {
  const layer = new Container()

  if (habitability.length === 0) return layer

  const scale = 4 // 下采样因子: 512 → 128
  const gw = Math.floor(width / scale)   // 128
  const gh = Math.floor(height / scale)  // 128

  const WORLD_MIN = -800
  const WORLD_MAX = 800
  const cellW = (WORLD_MAX - WORLD_MIN) / gw
  const cellH = (WORLD_MAX - WORLD_MIN) / gh

  // 预计算每个格子平均宜居度
  const grid: number[] = new Array(gw * gh).fill(0)
  const counts: number[] = new Array(gw * gh).fill(0)

  for (let py = 0; py < height; py++) {
    for (let px = 0; px < width; px++) {
      const gi = Math.floor(px / scale)
      const gj = Math.floor(py / scale)
      const gIdx = gj * gw + gi
      const hIdx = py * width + px
      if (hIdx < habitability.length) {
        grid[gIdx] += habitability[hIdx]
        counts[gIdx]++
      }
    }
  }

  // 批量绘制
  const g = new Graphics()

  for (let gj = 0; gj < gh; gj++) {
    for (let gi = 0; gi < gw; gi++) {
      const gIdx = gj * gw + gi
      if (counts[gIdx] === 0) continue

      const avg = grid[gIdx] / counts[gIdx]
      if (avg < 0.01) continue // 极低值跳过

      // 颜色映射: 0→0x440000(暗红), 0.5→0x888800(黄褐), 1→0x22aa44(绿)
      const color = heatColor(avg)
      const alpha = 0.15 + avg * 0.35 // 透明度随宜居度递增

      const wx = WORLD_MIN + gi * cellW
      const wy = WORLD_MIN + gj * cellH // py=0 → WORLD_MIN，与 contourExtractor/heightmap 一致

      g.rect(wx, wy, cellW, cellH)
      g.fill({ color, alpha })
    }
  }

  layer.addChild(g)
  return layer
}

/**
 * 宜居度 → 颜色
 * 低 (0.0): 暗红  0x441111
 * 中 (0.5): 黄褐  0x887722
 * 高 (1.0): 绿    0x33aa55
 */
function heatColor(value: number): number {
  const v = Math.max(0, Math.min(1, value))
  if (v < 0.5) {
    // 0.0 → 0.5: 暗红 → 黄褐
    const t = v / 0.5
    const r = Math.round(0x44 + (0x88 - 0x44) * t)
    const g = Math.round(0x11 + (0x77 - 0x11) * t)
    const b = Math.round(0x11 + (0x22 - 0x11) * t)
    return (r << 16) | (g << 8) | b
  } else {
    // 0.5 → 1.0: 黄褐 → 绿
    const t = (v - 0.5) / 0.5
    const r = Math.round(0x88 - 0x88 * t)
    const g = Math.round(0x77 + (0xaa - 0x77) * t)
    const b = Math.round(0x22 + (0x55 - 0x22) * t)
    return (r << 16) | (g << 8) | b
  }
}
