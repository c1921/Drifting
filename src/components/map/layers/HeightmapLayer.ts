import { Container, Graphics } from "pixi.js"

/**
 * 高度图灰度图层
 *
 * 将 512×512 高度数组下采样到 128×128，逐格绘制半透明灰度矩形。
 * 0.0 = 黑, 1.0 = 白，alpha ~0.35 以便上层元素可见。
 * 置于宜居度热力图层下方（作为最底层地形基底）。
 */
export function createHeightmapLayer(
  heightmap: number[],
  width: number,
  height: number,
): Container {
  const layer = new Container()

  if (heightmap.length === 0) return layer

  const scale = 4 // 下采样因子: 512 → 128
  const gw = Math.floor(width / scale)   // 128
  const gh = Math.floor(height / scale)  // 128

  const WORLD_MIN = -800
  const WORLD_MAX = 800
  const cellW = (WORLD_MAX - WORLD_MIN) / gw
  const cellH = (WORLD_MAX - WORLD_MIN) / gh

  // 预计算每个格子平均高度
  const grid: number[] = new Array(gw * gh).fill(0)
  const counts: number[] = new Array(gw * gh).fill(0)

  for (let py = 0; py < height; py++) {
    for (let px = 0; px < width; px++) {
      const gi = Math.floor(px / scale)
      const gj = Math.floor(py / scale)
      const gIdx = gj * gw + gi
      const hIdx = py * width + px
      if (hIdx < heightmap.length) {
        grid[gIdx] += heightmap[hIdx]
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

      // 灰度映射: 0→黑 0x000000, 1→白 0xffffff
      const brightness = Math.round(avg * 255)
      const color = (brightness << 16) | (brightness << 8) | brightness
      const alpha = 0.15 + avg * 0.25 // 半透明，避免完全遮盖下层

      const wx = WORLD_MIN + gi * cellW
      const wy = WORLD_MIN + gj * cellH

      g.rect(wx, wy, cellW, cellH)
      g.fill({ color, alpha })
    }
  }

  layer.addChild(g)
  return layer
}
