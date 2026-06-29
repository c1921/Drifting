import { Container, Graphics } from "pixi.js"

/**
 * 平坦区域中心度图层
 *
 * 将 512×512 平坦中心度数组下采样到 128×128，逐格绘制半透明矩形。
 * 颜色映射：0.0 = 蓝(低中心度), 1.0 = 橙黄(高中心度)。
 * 置于等高线层下方。
 */
export function createFlatCenterLayer(
  flatCenter: number[],
  width: number,
  height: number,
): Container {
  const layer = new Container()

  if (flatCenter.length === 0) return layer

  const scale = 4 // 下采样因子: 512 → 128
  const gw = Math.floor(width / scale)   // 128
  const gh = Math.floor(height / scale)  // 128

  const WORLD_MIN = -800
  const WORLD_MAX = 800
  const cellW = (WORLD_MAX - WORLD_MIN) / gw
  const cellH = (WORLD_MAX - WORLD_MIN) / gh

  // 预计算每个格子平均平坦中心度
  const grid: number[] = new Array(gw * gh).fill(0)
  const counts: number[] = new Array(gw * gh).fill(0)

  for (let py = 0; py < height; py++) {
    for (let px = 0; px < width; px++) {
      const gi = Math.floor(px / scale)
      const gj = Math.floor(py / scale)
      const gIdx = gj * gw + gi
      const hIdx = py * width + px
      if (hIdx < flatCenter.length) {
        grid[gIdx] += flatCenter[hIdx]
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
      if (avg < 0.01) continue

      const color = flatCenterColor(avg)
      const alpha = 0.15 + avg * 0.35

      const wx = WORLD_MIN + gi * cellW
      const wy = WORLD_MIN + gj * cellH

      g.rect(wx, wy, cellW, cellH)
      g.fill({ color, alpha })
    }
  }

  layer.addChild(g)
  return layer
}

/**
 * 平坦中心度 → 颜色
 * 低 (0.0): 深蓝  0x113355
 * 中 (0.5): 紫蓝  0x556688
 * 高 (1.0): 橙黄  0xcc8844
 */
function flatCenterColor(value: number): number {
  const v = Math.max(0, Math.min(1, value))
  if (v < 0.5) {
    // 0.0 → 0.5: 深蓝 → 紫蓝
    const t = v / 0.5
    const r = Math.round(0x11 + (0x55 - 0x11) * t)
    const g = Math.round(0x33 + (0x66 - 0x33) * t)
    const b = Math.round(0x55 + (0x88 - 0x55) * t)
    return (r << 16) | (g << 8) | b
  } else {
    // 0.5 → 1.0: 紫蓝 → 橙黄
    const t = (v - 0.5) / 0.5
    const r = Math.round(0x55 + (0xcc - 0x55) * t)
    const g = Math.round(0x66 + (0x88 - 0x66) * t)
    const b = Math.round(0x88 + (0x44 - 0x88) * t)
    return (r << 16) | (g << 8) | b
  }
}
