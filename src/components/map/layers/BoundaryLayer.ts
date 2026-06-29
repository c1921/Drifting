import { Container, Graphics } from "pixi.js"
import type { ThemeColors } from "../utils/themeUtils"

/**
 * 边界图层：绘制省界多边形
 * 在世界坐标系直接绘制，与 ContourLayer 一致。
 */
export function createBoundaryLayer(
  boundary: number[],
  colors: ThemeColors,
): Container | null {
  if (boundary.length < 6) return null // 至少 3 个点

  const layer = new Container()
  const g = new Graphics()

  // 画多边形
  g.moveTo(boundary[0], boundary[1])
  for (let i = 2; i < boundary.length - 1; i += 2) {
    g.lineTo(boundary[i], boundary[i + 1])
  }
  g.closePath()

  // 描边
  g.stroke({ width: 2.0, color: colors.primary, alpha: 0.5 })

  layer.addChild(g)
  return layer
}
