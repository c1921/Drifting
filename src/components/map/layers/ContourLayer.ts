import { Container, Graphics } from "pixi.js"
import type { ThemeColors } from "../utils/themeUtils"
import type { ContourData } from "@/map/contourExtractor"

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

    if (c.closed) {
      g.closePath()
    }

    // 填充（仅闭合环） + 描边
    const alpha = 0.04 + (i / contours.length) * 0.06
    if (c.closed) {
      g.fill({ color: colors.mutedForeground, alpha })
    }
    g.stroke({
      width: 0.5 + (i / contours.length) * 0.8,
      color: colors.mutedForeground,
      alpha: 0.15 + (i / contours.length) * 0.15,
    })

    layer.addChild(g)
  }

  return layer
}
